//! Variable-metric (Migrad) iteration loop with DFP update.
//!
//! Replaces VariableMetricBuilder.cxx and DavidonErrorUpdator.cxx. Contains
//! the main quasi-Newton iteration: compute step, line search, update gradient,
//! and apply the DFP rank-2 inverse Hessian update.

use nalgebra::{DMatrix, DVector};

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
    /// Returns the iteration history as a Vec<MinimumState>.
    pub fn minimum(
        fcn: &MnFcn,
        seed: &MinimumSeed,
        strategy: &MnStrategy,
        maxfcn: usize,
        edmval: f64,
    ) -> Vec<MinimumState> {
        // First pass: use full budget
        let states = Self::iterate(fcn, seed, strategy, maxfcn, edmval);

        if let Some(last) = states.last()
            && last.edm() < edmval
        {
            return states;
        }

        // If first pass failed and g2 has non-positive entries, we could re-seed.
        // (Full re-seeding with MnHesse is Phase 4 — for now, try second pass
        // with increased budget.)
        let maxfcn2 = (maxfcn as f64 * 1.3) as usize;
        let remaining = maxfcn2.saturating_sub(fcn.num_of_calls());
        if remaining < 10 {
            return states;
        }

        // Build a new seed from the last state
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

        let states2 = Self::iterate(fcn, &seed2, strategy, maxfcn2, edmval);
        if states2.is_empty() {
            states
        } else {
            states2
        }
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
        // First pass: use full budget
        let states = Self::iterate_with_gradient(fcn, gradient_fcn, seed, maxfcn, edmval);

        if let Some(last) = states.last()
            && last.edm() < edmval
        {
            return states;
        }

        // If first pass failed, try second pass with increased budget
        let maxfcn2 = (maxfcn as f64 * 1.3) as usize;
        let remaining = maxfcn2.saturating_sub(fcn.num_of_calls());
        if remaining < 10 {
            return states;
        }

        // Build a new seed from the last state
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

        let states2 = Self::iterate_with_gradient(fcn, gradient_fcn, &seed2, maxfcn2, edmval);
        if states2.is_empty() {
            states
        } else {
            states2
        }
    }

    /// Core iteration loop: Newton step → line search → gradient → DFP update.
    fn iterate(
        fcn: &MnFcn,
        seed: &MinimumSeed,
        strategy: &MnStrategy,
        maxfcn: usize,
        edmval: f64,
    ) -> Vec<MinimumState> {
        let n = seed.n_variable_params();
        let prec = seed.precision();

        // Current state
        let mut params = seed.parameters().clone();
        let mut error = seed.error().clone();
        let mut gradient = seed.gradient().clone();
        let mut edm = seed.edm();

        let grad_calc = Numerical2PGradientCalculator::new(*strategy);
        let mut states = Vec::new();

        loop {
            // 1. Newton step: step = -V * grad
            let v = error.matrix();
            let g = gradient.grad();
            let step = -(v * g);

            // 2. Check positive-definiteness: gdel = step · grad
            let mut gdel = step.dot(g);

            let (current_step, current_error) = if gdel > 0.0 {
                // step is not a descent direction — V is not pos.def.
                // Force V positive-definite and recompute
                let (v_fixed, _was_modified) = make_pos_def(v, prec);
                let mut err_fixed = MinimumError::new(v_fixed.clone(), error.dcovar());
                err_fixed.set_made_pos_def(true);
                let step_fixed = -(&v_fixed * g);
                gdel = step_fixed.dot(g);

                // If still not a descent direction, use steepest descent
                if gdel > 0.0 {
                    // Fall back to steepest descent with unit matrix
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

            // 3. Line search: parabolic interpolation along step
            let ls_result = mn_linesearch(fcn, &params, &current_step, gdel, prec);
            let lambda = ls_result.x;
            let f_new = ls_result.y;

            // Check for no improvement
            if (f_new - params.fval()).abs() <= params.fval().abs() * prec.eps() {
                // No significant improvement — likely at machine precision limit
                let new_params = MinimumParameters::with_step(
                    params.vec() + lambda * &current_step,
                    lambda * &current_step,
                    f_new,
                );
                let state = MinimumState::new(
                    new_params,
                    current_error.clone(),
                    gradient.clone(),
                    edm,
                    fcn.num_of_calls(),
                );
                states.push(state);
                break;
            }

            // 4. Update parameters: p_new = p_old + λ * step
            let p_new = params.vec() + lambda * &current_step;
            let new_params = MinimumParameters::with_step(
                p_new,
                lambda * &current_step,
                f_new,
            );

            // 5. Compute new gradient using previous gradient's step sizes
            let new_gradient = grad_calc.compute_with_previous(
                fcn,
                &new_params,
                seed.trafo(),
                &gradient,
            );

            // 6. DFP update of V
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

            // 7. EDM = 0.5 * g^T * V * g, corrected by (1 + 3*dcovar)
            let new_g = new_gradient.grad();
            let new_v = new_error.matrix();
            edm = 0.5 * new_g.dot(&(new_v * new_g));
            edm *= 1.0 + 3.0 * new_dcovar;

            // Save state
            let state = MinimumState::new(
                new_params.clone(),
                new_error.clone(),
                new_gradient.clone(),
                edm,
                fcn.num_of_calls(),
            );
            states.push(state);

            // 8. Check convergence
            if edm < edmval {
                break;
            }

            // Check call limit
            if fcn.num_of_calls() >= maxfcn {
                break;
            }

            // Update for next iteration
            params = new_params;
            error = new_error;
            gradient = new_gradient;
        }

        states
    }

    /// Core iteration loop with analytical gradients.
    fn iterate_with_gradient(
        fcn: &MnFcn,
        gradient_fcn: &dyn FCNGradient,
        seed: &MinimumSeed,
        maxfcn: usize,
        edmval: f64,
    ) -> Vec<MinimumState> {
        let n = seed.n_variable_params();
        let prec = seed.precision();

        // Current state
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

            // 2. Check positive-definiteness: gdel = step · grad
            let mut gdel = step.dot(g);

            let (current_step, current_error) = if gdel > 0.0 {
                // step is not a descent direction — V is not pos.def.
                // Force V positive-definite and recompute
                let (v_fixed, _was_modified) = make_pos_def(v, prec);
                let mut err_fixed = MinimumError::new(v_fixed.clone(), error.dcovar());
                err_fixed.set_made_pos_def(true);
                let step_fixed = -(&v_fixed * g);
                gdel = step_fixed.dot(g);

                // If still not a descent direction, use steepest descent
                if gdel > 0.0 {
                    // Fall back to steepest descent with unit matrix
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

            // 3. Line search: parabolic interpolation along step
            let ls_result = mn_linesearch(fcn, &params, &current_step, gdel, prec);
            let lambda = ls_result.x;
            let f_new = ls_result.y;

            // Check for no improvement
            if (f_new - params.fval()).abs() <= params.fval().abs() * prec.eps() {
                // No significant improvement — likely at machine precision limit
                let new_params = MinimumParameters::with_step(
                    params.vec() + lambda * &current_step,
                    lambda * &current_step,
                    f_new,
                );
                let state = MinimumState::new(
                    new_params,
                    current_error.clone(),
                    gradient.clone(),
                    edm,
                    fcn.num_of_calls(),
                );
                states.push(state);
                break;
            }

            // 4. Update parameters: p_new = p_old + λ * step
            let p_new = params.vec() + lambda * &current_step;
            let new_params = MinimumParameters::with_step(
                p_new,
                lambda * &current_step,
                f_new,
            );

            // 5. Compute new gradient using analytical gradient calculator
            let new_gradient = AnalyticalGradientCalculator::compute(
                gradient_fcn,
                seed.trafo(),
                &new_params,
            );

            // 6. DFP update of V
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

            // 7. EDM = 0.5 * g^T * V * g, corrected by (1 + 3*dcovar)
            let new_g = new_gradient.grad();
            let new_v = new_error.matrix();
            edm = 0.5 * new_g.dot(&(new_v * new_g));
            edm *= 1.0 + 3.0 * new_dcovar;

            // Save state
            let state = MinimumState::new(
                new_params.clone(),
                new_error.clone(),
                new_gradient.clone(),
                edm,
                fcn.num_of_calls(),
            );
            states.push(state);

            // 8. Check convergence
            if edm < edmval {
                break;
            }

            // Check call limit
            if fcn.num_of_calls() >= maxfcn {
                break;
            }

            // Update for next iteration
            params = new_params;
            error = new_error;
            gradient = new_gradient;
        }

        states
    }

    /// DFP (Davidon-Fletcher-Powell) rank-2 update of the inverse Hessian.
    ///
    /// Returns `(V_new, dcovar)`.
    fn dfp_update(
        error: &MinimumError,
        p_new: &MinimumParameters,
        p_old: &MinimumParameters,
        g_new: &FunctionGradient,
        g_old: &FunctionGradient,
    ) -> (DMatrix<f64>, f64) {
        let v = error.matrix();

        // dx = p_new - p_old (parameter change)
        let dx = p_new.vec() - p_old.vec();
        // dg = g_new - g_old (gradient change)
        let dg = g_new.grad() - g_old.grad();

        // delgam = dx · dg
        let delgam = dx.dot(&dg);

        // gvg = dg^T * V * dg
        let vg = v * &dg;
        let gvg = dg.dot(&vg);

        // Skip update if degenerate
        if delgam <= 0.0 || gvg <= 0.0 {
            return (v.clone(), error.dcovar());
        }

        // Rank-2 update: V_new = V + outer(dx)/delgam - outer(vg)/gvg
        let n = dx.len();
        let mut v_upd = DMatrix::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                v_upd[(i, j)] = dx[i] * dx[j] / delgam - vg[i] * vg[j] / gvg;
            }
        }

        // Rank-1 correction (BFGS-like): when delgam > gvg
        if delgam > gvg {
            // flnu = dx/delgam - vg/gvg
            let mut flnu = DVector::zeros(n);
            for i in 0..n {
                flnu[i] = dx[i] / delgam - vg[i] / gvg;
            }
            for i in 0..n {
                for j in 0..n {
                    v_upd[(i, j)] += gvg * flnu[i] * flnu[j];
                }
            }
        }

        let v_new = v + &v_upd;

        // dcovar: how much did V change?
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
