//! Nelder-Mead simplex iteration (Minuit variant).
//!
//! Replaces SimplexBuilder.cxx. This is NOT textbook Nelder-Mead --- it uses
//! a rho-based adaptive step inherited from the original Fortran MINUIT.
//!
//! Constants: alpha=1 (reflection), beta=0.5 (contraction), gamma=2 (expansion),
//! rhomin=4, rhomax=8.

use nalgebra::DVector;

use super::parameters::SimplexParameters;
use crate::minimum::parameters::MinimumParameters;
use crate::minimum::seed::MinimumSeed;
use crate::minimum::state::MinimumState;
use crate::mn_fcn::MnFcn;

pub struct SimplexBuilder;

impl SimplexBuilder {
    pub fn minimum(
        fcn: &MnFcn,
        seed: &MinimumSeed,
        maxfcn: usize,
        minedm: f64,
    ) -> Vec<MinimumState> {
        let n = seed.n_variable_params();
        let prec = seed.precision();

        let x = seed.parameters().vec().clone();
        let mut step: Vec<f64> = (0..n).map(|i| 10.0 * seed.gradient().gstep()[i]).collect();

        let alpha = 1.0_f64;
        let beta = 0.5_f64;
        let gamma = 2.0_f64;
        let rhomin = 4.0_f64;
        let rhomax = 8.0_f64;
        let rho1 = 1.0 + alpha;
        let rho2 = 1.0 + alpha * gamma;
        let wg = 1.0 / n as f64;

        // Build initial simplex: N+1 vertices
        let mut simpl: Vec<(f64, Vec<f64>)> = Vec::with_capacity(n + 1);
        simpl.push((seed.fval(), x.as_slice().to_vec()));

        let mut x_work = x.as_slice().to_vec();
        for i in 0..n {
            let dmin = 8.0 * prec.eps2() * (x_work[i].abs() + prec.eps2());
            if step[i] < dmin {
                step[i] = dmin;
            }
            x_work[i] += step[i];
            let fval = fcn.call(&x_work);
            simpl.push((fval, x_work.clone()));
            x_work[i] -= step[i]; // restore
        }

        let mut simplex = SimplexParameters::new(simpl);
        // Main iteration loop (do-while in C++)
        // edm_prev tracks the EDM from the previous iteration — both must
        // be below threshold for convergence (prevents premature stop).
        let mut edm_prev;
        loop {
            let jh = simplex.jhigh();
            let amin = simplex.fval_best();
            edm_prev = simplex.edm();

            // Compute centroid (pbar) excluding worst vertex
            let mut pbar = vec![0.0; n];
            for (i, (_, v)) in simplex.params().iter().enumerate() {
                if i == jh {
                    continue;
                }
                for j in 0..n {
                    pbar[j] += wg * v[j];
                }
            }

            // Reflect: pstar = (1+alpha)*pbar - alpha*worst
            let worst = simplex.params()[jh].1.clone();
            let mut pstar = vec![0.0; n];
            for j in 0..n {
                pstar[j] = (1.0 + alpha) * pbar[j] - alpha * worst[j];
            }
            let ystar = fcn.call(&pstar);

            if ystar > amin {
                // Reflected point is worse than best
                if ystar < simplex.params()[jh].0 {
                    // But better than worst — accept and check
                    simplex.update(jh, ystar, pstar.clone());
                    if jh != simplex.jhigh() {
                        // Worst vertex changed, continue iteration
                        if !Self::should_stop(&simplex, edm_prev, minedm, fcn, maxfcn) {
                            continue;
                        }
                        break;
                    }
                }
                // Contraction: pstst = beta*worst + (1-beta)*pbar
                let worst_cur = simplex.params()[simplex.jhigh()].1.clone();
                let mut pstst = vec![0.0; n];
                for j in 0..n {
                    pstst[j] = beta * worst_cur[j] + (1.0 - beta) * pbar[j];
                }
                let ystst = fcn.call(&pstst);

                if ystst > simplex.params()[simplex.jhigh()].0 {
                    // Contraction failed — stop
                    break;
                }
                simplex.update(simplex.jhigh(), ystst, pstst);
            } else {
                // ystar < amin: reflected is better than best
                // Try expansion: pstst = gamma*pstar + (1-gamma)*pbar
                let mut pstst = vec![0.0; n];
                for j in 0..n {
                    pstst[j] = gamma * pstar[j] + (1.0 - gamma) * pbar[j];
                }
                let ystst = fcn.call(&pstst);

                // Rho extrapolation (Minuit-specific optimization)
                let y1 = (ystar - simplex.params()[jh].0) * rho2;
                let y2 = (ystst - simplex.params()[jh].0) * rho1;
                let denom = y1 - y2;

                if denom.abs() < 1e-30 {
                    // Degenerate — pick the better of expansion/reflection
                    if ystst < simplex.fval_best() {
                        simplex.update(jh, ystst, pstst);
                    } else {
                        simplex.update(jh, ystar, pstar);
                    }
                } else {
                    let rho = 0.5 * (rho2 * y1 - rho1 * y2) / denom;

                    if rho < rhomin {
                        if ystst < simplex.fval_best() {
                            simplex.update(jh, ystst, pstst);
                        } else {
                            simplex.update(jh, ystar, pstar);
                        }
                    } else {
                        let rho_clamped = rho.min(rhomax);
                        let mut prho = vec![0.0; n];
                        for j in 0..n {
                            prho[j] = rho_clamped * pbar[j] + (1.0 - rho_clamped) * worst[j];
                        }
                        let yrho = fcn.call(&prho);

                        if yrho < simplex.fval_best() && yrho < ystst {
                            simplex.update(jh, yrho, prho);
                        } else if ystst < simplex.fval_best() {
                            simplex.update(jh, ystst, pstst);
                        } else if yrho > simplex.fval_best() {
                            if ystst < simplex.fval_best() {
                                simplex.update(jh, ystst, pstst);
                            } else {
                                simplex.update(jh, ystar, pstar);
                            }
                        } else {
                            simplex.update(jh, ystar, pstar);
                        }
                    }
                }
            }

            // Check convergence at end of iteration (do-while)
            if Self::should_stop(&simplex, edm_prev, minedm, fcn, maxfcn) {
                break;
            }
        }

        // Post-loop: try centroid as final point
        let jh = simplex.jhigh();
        let mut pbar = vec![0.0; n];
        for (i, (_, v)) in simplex.params().iter().enumerate() {
            if i == jh {
                continue;
            }
            for j in 0..n {
                pbar[j] += wg * v[j];
            }
        }
        let ybar = fcn.call(&pbar);

        let (final_vec, final_fval) = if ybar < simplex.fval_best() {
            simplex.update(simplex.jhigh(), ybar, pbar.clone());
            (pbar, ybar)
        } else {
            (simplex.best().to_vec(), simplex.fval_best())
        };

        // Compute dirin from simplex spread, scaled by sqrt(up/edm)
        let edm = simplex.edm();
        let up = fcn.up();
        let scale = if edm > f64::MIN_POSITIVE {
            (up / edm).sqrt()
        } else {
            1.0
        };

        let mut dirin = vec![0.0; n];
        for i in 0..n {
            let mut lo = f64::MAX;
            let mut hi = f64::MIN;
            for (_, v) in simplex.params().iter() {
                if v[i] < lo {
                    lo = v[i];
                }
                if v[i] > hi {
                    hi = v[i];
                }
            }
            dirin[i] = (hi - lo) * scale;
        }

        let final_params = MinimumParameters::with_step(
            DVector::from_vec(final_vec),
            DVector::from_vec(dirin),
            final_fval,
        );

        let state = MinimumState::from_params_edm(final_params, edm, fcn.num_of_calls());
        vec![state]
    }

    fn should_stop(
        simplex: &SimplexParameters,
        edm_prev: f64,
        minedm: f64,
        fcn: &MnFcn,
        maxfcn: usize,
    ) -> bool {
        if fcn.num_of_calls() >= maxfcn {
            return true;
        }
        // Both current and previous EDM must be below threshold
        simplex.edm() <= minedm && edm_prev <= minedm
    }
}
