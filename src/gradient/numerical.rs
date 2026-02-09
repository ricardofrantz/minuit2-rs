//! Two-point numerical gradient calculator.
//!
//! Replaces Numerical2PGradientCalculator.cxx. Computes the gradient using
//! central differences: g_i = (f(x+h) - f(x-h)) / 2h, with adaptive step
//! sizing refined over multiple cycles. Also computes g2 (second derivative
//! estimate) and gstep (optimal step size).

use nalgebra::DVector;

use crate::minimum::gradient::FunctionGradient;
use crate::minimum::parameters::MinimumParameters;
use crate::mn_fcn::MnFcn;
use crate::strategy::MnStrategy;
use crate::user_transformation::MnUserTransformation;

pub struct Numerical2PGradientCalculator {
    strategy: MnStrategy,
}

impl Numerical2PGradientCalculator {
    pub fn new(strategy: MnStrategy) -> Self {
        Self { strategy }
    }

    /// Compute gradient from scratch (no previous gradient available).
    /// Uses the heuristic gradient's gstep as initial step sizes.
    pub fn compute(
        &self,
        fcn: &MnFcn,
        params: &MinimumParameters,
        trafo: &MnUserTransformation,
        initial_gradient: &FunctionGradient,
    ) -> FunctionGradient {
        let n = trafo.variable_parameters();
        let eps2 = trafo.precision().eps2();
        let fcnmin = params.fval();
        let dfmin = 8.0 * eps2 * (fcnmin.abs() + fcn.up());
        let vrysml = 8.0 * eps2 * eps2;

        let x = params.vec();
        let ncycles = self.strategy.grad_ncycles();
        let step_tol = self.strategy.grad_step_tol();
        let grad_tol = self.strategy.grad_tol();

        let mut grad = DVector::zeros(n);
        let mut g2 = DVector::zeros(n);
        let mut gstep = DVector::zeros(n);

        for i in 0..n {
            let ext_idx = trafo.ext_of_int(i);
            let xi = x[i];
            let p = &trafo.parameters()[ext_idx];
            let has_limits = p.has_limits() || p.has_lower_limit() || p.has_upper_limit();

            // Initial step from heuristic gradient
            let mut gstepi = initial_gradient.gstep()[i].max(vrysml);
            let mut g2i = initial_gradient.g2()[i];

            // Ncycles of refinement
            for _cycle in 0..ncycles {
                // Optimal step: balance truncation vs roundoff error
                let optstp = (dfmin / (g2i.abs() + eps2)).sqrt();
                let mut step = optstp.max(0.1 * gstepi.abs());

                // Bounded parameter: cap step at 0.5
                if has_limits {
                    step = step.min(0.5);
                }

                // Clamp step
                let stpmax = 10.0 * gstepi.abs();
                let stpmin = vrysml.max(8.0 * eps2 * xi.abs());
                step = step.clamp(stpmin, stpmax);

                let stepb4 = gstepi;
                let grdb4 = grad[i];

                gstepi = step;

                // Central differences: f(x+h) - f(x-h)
                let mut xp = x.clone();
                let mut xm = x.clone();
                xp[i] = xi + step;
                xm[i] = xi - step;

                let fp = fcn.call(xp.as_slice());
                let fm = fcn.call(xm.as_slice());

                let grdi = 0.5 * (fp - fm) / step;
                let g2i_new = (fp + fm - 2.0 * fcnmin) / (step * step);

                grad[i] = grdi;
                g2[i] = g2i_new;
                gstep[i] = gstepi;
                g2i = g2i_new;

                // Check convergence: step stabilization
                if _cycle > 0 {
                    let step_change = (gstepi - stepb4).abs() / gstepi.abs();
                    if step_change < step_tol {
                        break;
                    }

                    // Gradient stabilization
                    let grad_change = (grdi - grdb4).abs() / (grdi.abs() + dfmin / step);
                    if grad_change < grad_tol {
                        break;
                    }
                }
            }
        }

        FunctionGradient::new(grad, g2, gstep)
    }

    /// Compute gradient using previous gradient's step sizes as starting point.
    /// More efficient than `compute()` since step sizes are already tuned.
    pub fn compute_with_previous(
        &self,
        fcn: &MnFcn,
        params: &MinimumParameters,
        trafo: &MnUserTransformation,
        previous: &FunctionGradient,
    ) -> FunctionGradient {
        let n = trafo.variable_parameters();
        let eps2 = trafo.precision().eps2();
        let fcnmin = params.fval();
        let dfmin = 8.0 * eps2 * (fcnmin.abs() + fcn.up());
        let vrysml = 8.0 * eps2 * eps2;

        let x = params.vec();
        let ncycles = self.strategy.grad_ncycles();
        let step_tol = self.strategy.grad_step_tol();
        let grad_tol = self.strategy.grad_tol();

        let mut grad = DVector::zeros(n);
        let mut g2 = DVector::zeros(n);
        let mut gstep = DVector::zeros(n);

        for i in 0..n {
            let ext_idx = trafo.ext_of_int(i);
            let xi = x[i];
            let p = &trafo.parameters()[ext_idx];
            let has_limits = p.has_limits() || p.has_lower_limit() || p.has_upper_limit();

            // Start from previous step sizes
            let mut gstepi = previous.gstep()[i].max(vrysml);
            let mut g2i = previous.g2()[i];

            for _cycle in 0..ncycles {
                let optstp = (dfmin / (g2i.abs() + eps2)).sqrt();
                let mut step = optstp.max(0.1 * gstepi.abs());

                if has_limits {
                    step = step.min(0.5);
                }

                let stpmax = 10.0 * gstepi.abs();
                let stpmin = vrysml.max(8.0 * eps2 * xi.abs());
                step = step.clamp(stpmin, stpmax);

                let stepb4 = gstepi;
                let grdb4 = grad[i];

                gstepi = step;

                let mut xp = x.clone();
                let mut xm = x.clone();
                xp[i] = xi + step;
                xm[i] = xi - step;

                let fp = fcn.call(xp.as_slice());
                let fm = fcn.call(xm.as_slice());

                let grdi = 0.5 * (fp - fm) / step;
                let g2i_new = (fp + fm - 2.0 * fcnmin) / (step * step);

                grad[i] = grdi;
                g2[i] = g2i_new;
                gstep[i] = gstepi;
                g2i = g2i_new;

                if _cycle > 0 {
                    let step_change = (gstepi - stepb4).abs() / gstepi.abs();
                    if step_change < step_tol {
                        break;
                    }

                    let grad_change = (grdi - grdb4).abs() / (grdi.abs() + dfmin / step);
                    if grad_change < grad_tol {
                        break;
                    }
                }
            }
        }

        FunctionGradient::new(grad, g2, gstep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fcn::FCN;
    use crate::parameter::MinuitParameter;
    use crate::user_transformation::MnUserTransformation;

    struct Quadratic;
    impl FCN for Quadratic {
        fn value(&self, p: &[f64]) -> f64 {
            // f(x,y) = x² + 4y²
            p[0] * p[0] + 4.0 * p[1] * p[1]
        }
    }

    #[test]
    fn numerical_gradient_quadratic() {
        let params = vec![
            MinuitParameter::new(0, "x", 3.0, 0.1),
            MinuitParameter::new(1, "y", 2.0, 0.1),
        ];
        let trafo = MnUserTransformation::new(params);
        let fcn = MnFcn::new(&Quadratic, &trafo);
        let strategy = MnStrategy::default();

        // Evaluate at (3, 2) → f = 9 + 16 = 25
        let x = DVector::from_vec(vec![3.0, 2.0]);
        let min_params = MinimumParameters::new(x, 25.0);

        // Heuristic initial gradient (just for step sizes)
        let ig = crate::gradient::InitialGradientCalculator::new(strategy);
        let init_grad = ig.compute(&fcn, &min_params, &trafo);

        let calc = Numerical2PGradientCalculator::new(strategy);
        let grad = calc.compute(&fcn, &min_params, &trafo, &init_grad);

        // df/dx = 2x = 6, df/dy = 8y = 16
        assert!(
            (grad.grad()[0] - 6.0).abs() < 0.01,
            "dfdx should be ~6.0, got {}",
            grad.grad()[0]
        );
        assert!(
            (grad.grad()[1] - 16.0).abs() < 0.1,
            "dfdy should be ~16.0, got {}",
            grad.grad()[1]
        );
    }
}
