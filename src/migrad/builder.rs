//! Variable-metric (Migrad) iteration loop with DFP update.
//!
//! Main quasi-Newton iteration: compute step, line search, update gradient,
//! and apply the DFP rank-2 inverse Hessian update.

use nalgebra::DMatrix;

use crate::fcn::FCNGradient;
use crate::gradient::{AnalyticalGradientCalculator, Numerical2PGradientCalculator};
use crate::linesearch::mn_linesearch;
use crate::minimum::error::{ErrorMatrixStatus, MinimumError};
use crate::minimum::gradient::FunctionGradient;
use crate::minimum::parameters::MinimumParameters;
use crate::minimum::seed::MinimumSeed;
use crate::minimum::state::MinimumState;
use crate::mn_fcn::MnFcn;
use crate::posdef::make_pos_def;
use crate::strategy::MnStrategy;

pub struct VariableMetricBuilder;

impl VariableMetricBuilder {
    /// Top-level Migrad minimization: run iterations, optionally re-seed on failure.
    ///
    /// Returns the iteration history as a `Vec<MinimumState>`.
    pub fn minimum(
        fcn: &MnFcn,
        seed: &MinimumSeed,
        strategy: &MnStrategy,
        maxfcn: usize,
        edmval: f64,
    ) -> Vec<MinimumState> {
        let grad_calc = Numerical2PGradientCalculator::new(*strategy);
        let next_grad = |p: &MinimumParameters, prev: &FunctionGradient| {
            grad_calc.compute_with_previous(fcn, p, seed.trafo(), prev)
        };
        Self::minimize_with_reseed(fcn, seed, maxfcn, edmval, next_grad)
    }

    /// Top-level Migrad minimization with analytical gradients.
    pub fn minimum_with_gradient(
        fcn: &MnFcn,
        gradient_fcn: &dyn FCNGradient,
        seed: &MinimumSeed,
        _strategy: &MnStrategy,
        maxfcn: usize,
        edmval: f64,
    ) -> Vec<MinimumState> {
        let next_grad = |p: &MinimumParameters, _prev: &FunctionGradient| {
            AnalyticalGradientCalculator::compute(gradient_fcn, seed.trafo(), p)
        };
        Self::minimize_with_reseed(fcn, seed, maxfcn, edmval, next_grad)
    }

    /// Run one or two passes of the quasi-Newton loop, re-seeding from the last
    /// state if the first pass did not converge and budget remains.
    ///
    /// `next_grad(new_params, prev_grad)` computes the gradient at `new_params`;
    /// the numerical strategy uses `prev_grad` for step-size warm-starting while
    /// the analytical strategy ignores it.
    fn minimize_with_reseed(
        fcn: &MnFcn,
        seed: &MinimumSeed,
        maxfcn: usize,
        edmval: f64,
        mut next_grad: impl FnMut(&MinimumParameters, &FunctionGradient) -> FunctionGradient,
    ) -> Vec<MinimumState> {
        let states = Self::iterate(fcn, seed, maxfcn, edmval, &mut next_grad);

        if let Some(last) = states.last()
            && last.edm() < edmval
        {
            return states;
        }

        let maxfcn2 = (maxfcn as f64 * 1.3) as usize;
        let remaining = maxfcn2.saturating_sub(fcn.num_of_calls());
        if remaining < 10 {
            return states;
        }

        let last = states.last().unwrap_or_else(|| seed.state());
        let seed2 = MinimumSeed::new(
            MinimumState::new(
                last.parameters().clone(),
                last.error().clone(),
                last.gradient().clone(),
                last.edm(),
                last.nfcn(),
            ),
            seed.trafo().clone(),
        );

        let states2 = Self::iterate(fcn, &seed2, maxfcn2, edmval, &mut next_grad);
        if states2.is_empty() { states } else { states2 }
    }

    /// Core quasi-Newton iteration: Newton step → pos-def fallback → line search
    /// → gradient update (via `next_grad`) → DFP update → EDM check.
    fn iterate(
        fcn: &MnFcn,
        seed: &MinimumSeed,
        maxfcn: usize,
        edmval: f64,
        next_grad: &mut impl FnMut(&MinimumParameters, &FunctionGradient) -> FunctionGradient,
    ) -> Vec<MinimumState> {
        let n = seed.n_variable_params();
        let prec = seed.precision();

        let mut params = seed.parameters().clone();
        let mut error = seed.error().clone();
        let mut gradient = seed.gradient().clone();
        let mut edm = seed.edm();

        let mut states = Vec::new();

        loop {
            // 1. Newton step: step = -V * grad
            let v = error.matrix();
            let g = gradient.grad();
            let step = -(v * g);

            // 2. Ensure descent direction: gdel = step · grad must be negative.
            //    If not, make V positive-definite; if still bad, fall back to
            //    steepest descent with the identity matrix.
            let mut gdel = step.dot(g);

            let (current_step, current_error) = if gdel > 0.0 {
                let (v_fixed, _was_modified) = make_pos_def(v, prec);
                let mut err_fixed = MinimumError::new(v_fixed.clone(), error.dcovar());
                err_fixed.set_made_pos_def(true);
                let step_fixed = -(&v_fixed * g);
                gdel = step_fixed.dot(g);

                if gdel > 0.0 {
                    let step_sd = -g.clone();
                    gdel = step_sd.dot(g);
                    let err_sd = MinimumError::new(DMatrix::identity(n, n), 1.0);
                    (step_sd, err_sd)
                } else {
                    (step_fixed, err_fixed)
                }
            } else {
                (step, error.clone())
            };

            // 3. Line search: parabolic interpolation along current_step
            let ls_result = mn_linesearch(fcn, &params, &current_step, gdel, prec);
            let lambda = ls_result.x;
            let f_new = ls_result.y;

            // 4. No-improvement guard: stop if we are at machine-precision plateau
            if (f_new - params.fval()).abs() <= params.fval().abs() * prec.eps() {
                let new_params = MinimumParameters::with_step(
                    params.vec() + lambda * &current_step,
                    lambda * &current_step,
                    f_new,
                );
                states.push(MinimumState::new(
                    new_params,
                    current_error.clone(),
                    gradient.clone(),
                    edm,
                    fcn.num_of_calls(),
                ));
                break;
            }

            // 5. Advance parameters
            let p_new = params.vec() + lambda * &current_step;
            let new_params = MinimumParameters::with_step(p_new, lambda * &current_step, f_new);

            // 6. Compute gradient at the new point
            let new_gradient = next_grad(&new_params, &gradient);

            // 7. DFP rank-2 update of the inverse Hessian approximation
            let (v_updated, new_dcovar) = Self::dfp_update(
                &current_error,
                &new_params,
                &params,
                &new_gradient,
                &gradient,
            );

            let mut new_error = MinimumError::new(v_updated, new_dcovar);
            if current_error.status() == ErrorMatrixStatus::MadePositiveDefinite {
                new_error.set_made_pos_def(true);
            }

            // 8. EDM = 0.5 * g^T * V * g, scaled by matrix quality factor
            let new_g = new_gradient.grad();
            let new_v = new_error.matrix();
            edm = 0.5 * new_g.dot(&(new_v * new_g));
            edm *= 1.0 + 3.0 * new_dcovar;

            states.push(MinimumState::new(
                new_params.clone(),
                new_error.clone(),
                new_gradient.clone(),
                edm,
                fcn.num_of_calls(),
            ));

            if edm < edmval {
                break;
            }

            if fcn.num_of_calls() >= maxfcn {
                break;
            }

            params = new_params;
            error = new_error;
            gradient = new_gradient;
        }

        states
    }

    /// Rank-2 DFP update of the inverse Hessian approximation.
    ///
    /// Returns `(V_new, dcovar)` where `dcovar` measures how much the matrix changed.
    pub fn update(
        error: &MinimumError,
        p_new: &MinimumParameters,
        p_old: &MinimumParameters,
        g_new: &FunctionGradient,
        g_old: &FunctionGradient,
    ) -> (DMatrix<f64>, f64) {
        Self::dfp_update(error, p_new, p_old, g_new, g_old)
    }

    pub fn nrow(error: &MinimumError) -> usize {
        error.matrix().nrows()
    }

    pub fn has_negative_g2(gradient: &FunctionGradient) -> bool {
        gradient.g2().iter().any(|g2| *g2 <= 0.0)
    }

    pub fn add_result(states: &mut Vec<MinimumState>, state: MinimumState) {
        states.push(state);
    }

    fn dfp_update(
        error: &MinimumError,
        p_new: &MinimumParameters,
        p_old: &MinimumParameters,
        g_new: &FunctionGradient,
        g_old: &FunctionGradient,
    ) -> (DMatrix<f64>, f64) {
        let v = error.matrix();

        let dx = p_new.vec() - p_old.vec();
        let dg = g_new.grad() - g_old.grad();

        // delgam = dx · dg  (curvature along the step)
        let delgam = dx.dot(&dg);

        // vg = V * dg;  gvg = dg^T * V * dg
        let vg = v * &dg;
        let gvg = dg.dot(&vg);

        if delgam <= 0.0 || gvg <= 0.0 {
            return (v.clone(), error.dcovar());
        }

        // Rank-2 update matrix: outer(dx, dx)/delgam - outer(vg, vg)/gvg
        let mut v_upd = &dx * dx.transpose() / delgam - &vg * vg.transpose() / gvg;

        // Optional rank-1 correction when curvature condition is strong
        if delgam > gvg {
            let flnu = &dx / delgam - &vg / gvg;
            // Scale flnu by gvg before the outer product so each element is
            // (gvg * flnu[i]) * flnu[j], matching the original left-to-right
            // associativity exactly (f64 multiplication is not associative).
            v_upd += (&flnu * gvg) * flnu.transpose();
        }

        let v_new = v + &v_upd;

        let sum_upd: f64 = v_upd.iter().map(|x| x.abs()).sum();
        let sum_new: f64 = v_new.iter().map(|x| x.abs()).sum();
        let dcovar = if sum_new > 0.0 {
            0.5 * (error.dcovar() + sum_upd / sum_new)
        } else {
            error.dcovar()
        };

        (v_new, dcovar)
    }
}
