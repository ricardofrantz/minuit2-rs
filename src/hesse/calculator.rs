//! Core Hesse algorithm: computes the full Hessian matrix by finite differences.
//!
//! Replaces HesseCrossDerivative.cxx and the diagonal Hessian calculation from
//! MnHesse.cxx. Steps:
//! 1. Diagonal elements via 5-point refinement
//! 2. Gradient refinement using Hessian info (if strategy > 0)
//! 3. Off-diagonal elements via cross-derivatives
//! 4. Make positive-definite
//! 5. Invert Hessian → covariance

use nalgebra::{DMatrix, DVector};

use crate::minimum::error::{ErrorMatrixStatus, MinimumError};
use crate::minimum::gradient::FunctionGradient;
use crate::minimum::state::MinimumState;
use crate::mn_fcn::MnFcn;
use crate::posdef::make_pos_def;
use crate::strategy::MnStrategy;
use crate::user_transformation::MnUserTransformation;

use super::gradient::HessianGradientCalculator;

/// Result of a Hesse calculation.
pub struct HesseResult {
    pub state: MinimumState,
    pub hesse_failed: bool,
    pub invert_failed: bool,
    pub made_pos_def: bool,
}

/// Run the full Hesse algorithm.
///
/// Computes the second derivative matrix (Hessian) at the minimum using
/// finite differences, inverts to get the covariance, and returns an updated
/// MinimumState.
pub fn calculate(
    fcn: &MnFcn,
    state: &MinimumState,
    trafo: &MnUserTransformation,
    strategy: &MnStrategy,
    maxcalls: usize,
) -> HesseResult {
    let n = trafo.variable_parameters();
    let eps2 = trafo.precision().eps2();
    let up = fcn.up();
    let amin = state.fval();

    let x = state.parameters().vec().clone();
    let ncycles = strategy.hess_ncycles();
    let hess_step_tol = strategy.hess_step_tol();
    let hess_g2_tol = strategy.hess_g2_tol();

    // Starting gradient info
    let g = state.gradient();
    let mut g2 = g.g2().clone();
    let mut gstep = g.gstep().clone();
    let mut grad = g.grad().clone();

    // --- Step 1: Diagonal Hessian elements ---
    let mut hessian_g2 = DVector::zeros(n);
    let mut hessian_gstep = DVector::zeros(n);
    let mut hesse_failed = false;

    for i in 0..n {
        if fcn.num_of_calls() >= maxcalls {
            break;
        }

        let ext_idx = trafo.ext_of_int(i);
        let xi = x[i];
        let p = &trafo.parameters()[ext_idx];
        let has_limits = p.has_limits() || p.has_lower_limit() || p.has_upper_limit();

        let dmin = 8.0 * eps2 * (xi.abs() + eps2);
        let aimsag = (eps2).sqrt() * (amin.abs() + up);

        // Start with current step or compute from g2
        let mut d = if g2[i].abs() > eps2 {
            (8.0 * aimsag / g2[i].abs()).sqrt().max(dmin)
        } else {
            dmin
        };

        // Bounded params: cap step at 0.5
        if has_limits {
            d = d.min(0.5);
        }

        let mut g2i = g2[i];

        for _cycle in 0..ncycles as usize {
            if fcn.num_of_calls() >= maxcalls {
                break;
            }

            let dlast = d;
            let g2bfr = g2i;

            // 5-point: evaluate at x+d and x-d
            let mut xp = x.clone();
            let mut xm = x.clone();
            xp[i] = xi + d;
            xm[i] = xi - d;

            let fp = fcn.call(xp.as_slice());
            let fm = fcn.call(xm.as_slice());

            let sag = 0.5 * (fp + fm - 2.0 * amin);

            if sag.abs() < eps2 {
                // sag too small — increase step
                if g2i < eps2 {
                    // Hesse failed for this parameter — can't determine curvature
                    hesse_failed = true;
                }
                d *= 10.0;
                if has_limits {
                    d = d.min(0.5);
                }
                continue;
            }

            g2i = 2.0 * sag / (d * d);

            // Update gradient estimate from same evaluations
            let grdi = 0.5 * (fp - fm) / d;
            grad[i] = grdi;

            // Adaptive step from sag
            d *= (aimsag / sag.abs()).sqrt();
            d = d.max(dmin);
            if has_limits {
                d = d.min(0.5);
            }

            // Convergence check
            if _cycle > 0 {
                let d_change = (d - dlast).abs() / d;
                let g2_change = (g2i - g2bfr).abs() / g2i.abs();
                if d_change < hess_step_tol && g2_change < hess_g2_tol {
                    break;
                }
            }
        }

        hessian_g2[i] = g2i;
        hessian_gstep[i] = d;
        g2[i] = g2i;
        gstep[i] = d;
    }

    // --- Step 2: Refine gradient using Hessian info (strategy > 0) ---
    if strategy.strategy() > 0 && !hesse_failed {
        let refined_grad = HessianGradientCalculator::compute(
            fcn,
            state.parameters(),
            trafo,
            strategy,
            &hessian_g2,
            &hessian_gstep,
        );
        grad = refined_grad.grad().clone();
        g2 = refined_grad.g2().clone();
        gstep = refined_grad.gstep().clone();
    }

    // --- Step 3: Off-diagonal Hessian elements ---
    let mut hessian = DMatrix::zeros(n, n);

    // Fill diagonal
    for i in 0..n {
        hessian[(i, i)] = hessian_g2[i];
    }

    // Off-diagonal: H(i,j) = (f(x+di*ei+dj*ej) + f0 - f(x+di*ei) - f(x+dj*ej)) / (di*dj)
    for i in 0..n {
        for j in (i + 1)..n {
            if fcn.num_of_calls() >= maxcalls {
                break;
            }

            let di = hessian_gstep[i];
            let dj = hessian_gstep[j];

            let mut xpp = x.clone();
            xpp[i] += di;
            xpp[j] += dj;
            let fpp = fcn.call(xpp.as_slice());

            let mut xpi = x.clone();
            xpi[i] += di;
            let fpi = fcn.call(xpi.as_slice());

            let mut xpj = x.clone();
            xpj[j] += dj;
            let fpj = fcn.call(xpj.as_slice());

            let cross = (fpp + amin - fpi - fpj) / (di * dj);
            hessian[(i, j)] = cross;
            hessian[(j, i)] = cross;
        }
    }

    // --- Step 4: Make positive-definite ---
    let (hessian_pd, was_modified) = make_pos_def(&hessian, trafo.precision());

    // --- Step 5: Invert Hessian → covariance ---
    let (error, invert_failed) = match hessian_pd.clone().try_inverse() {
        Some(cov) => {
            let mut err = MinimumError::new(cov, 0.0);
            if was_modified {
                err.set_made_pos_def(true);
            }
            if hesse_failed {
                err.set_hesse_failed(true);
            }
            if !hesse_failed && !was_modified {
                err.set_status(ErrorMatrixStatus::Accurate);
            }
            (err, false)
        }
        None => {
            // Inversion failed — return diagonal of 1/H_ii
            let mut diag = DMatrix::zeros(n, n);
            for i in 0..n {
                if hessian_pd[(i, i)].abs() > eps2 {
                    diag[(i, i)] = 1.0 / hessian_pd[(i, i)];
                } else {
                    diag[(i, i)] = 1.0;
                }
            }
            let mut err = MinimumError::new(diag, 1.0);
            err.set_invert_failed(true);
            (err, true)
        }
    };

    // --- Step 6: EDM = 0.5 * g^T * V * g ---
    let gradient = FunctionGradient::new(grad.clone(), g2, gstep);
    let edm = {
        let g = gradient.grad();
        let e = error.matrix();
        0.5 * g.dot(&(e * g))
    };

    let new_state = MinimumState::new(
        state.parameters().clone(),
        error,
        gradient,
        edm,
        fcn.num_of_calls(),
    );

    HesseResult {
        state: new_state,
        hesse_failed,
        invert_failed,
        made_pos_def: was_modified,
    }
}
