//! Migrad seed generator.
//!
//! Replaces MnSeedGenerator.cxx (Migrad path). Creates the initial MinimumSeed
//! by evaluating the FCN, computing a numerical gradient (not just heuristic),
//! and building V₀ = diag(1/g2_i).

use nalgebra::{DMatrix, DVector};

use crate::fcn::FCNGradient;
use crate::gradient::{AnalyticalGradientCalculator, InitialGradientCalculator, Numerical2PGradientCalculator};
use crate::minimum::error::MinimumError;
use crate::minimum::parameters::MinimumParameters;
use crate::minimum::seed::MinimumSeed;
use crate::minimum::state::MinimumState;
use crate::mn_fcn::MnFcn;
use crate::strategy::MnStrategy;
use crate::user_transformation::MnUserTransformation;

pub struct MigradSeedGenerator;

impl MigradSeedGenerator {
    /// Generate seed using numerical gradients (central differences).
    pub fn generate(
        fcn: &MnFcn,
        trafo: &MnUserTransformation,
        strategy: &MnStrategy,
    ) -> MinimumSeed {
        let n = trafo.variable_parameters();
        let eps2 = trafo.precision().eps2();

        // 1. Get initial internal parameter values
        let int_values = trafo.initial_internal_values();
        let int_vec = DVector::from_vec(int_values.clone());

        // 2. Evaluate FCN at starting point
        let fval = fcn.call(&int_values);
        let params = MinimumParameters::new(int_vec, fval);

        // 3. Compute heuristic gradient (no FCN calls — just from step sizes)
        let heuristic_calc = InitialGradientCalculator::new(*strategy);
        let heuristic_grad = heuristic_calc.compute(fcn, &params, trafo);

        // 4. Compute numerical gradient (2-point central differences)
        let numerical_calc = Numerical2PGradientCalculator::new(*strategy);
        let gradient = numerical_calc.compute(fcn, &params, trafo, &heuristic_grad);

        // 5. Build V₀ = diag(1/g2_i), fallback to 1.0 for non-positive g2
        let mut v0 = DMatrix::zeros(n, n);
        for i in 0..n {
            let g2i = gradient.g2()[i];
            v0[(i, i)] = if g2i > eps2 {
                1.0 / g2i
            } else {
                1.0
            };
        }

        let dcovar = 1.0; // approximate: initial V is rough
        let error = MinimumError::new(v0, dcovar);

        // 6. EDM = 0.5 * g^T * V * g
        let edm = {
            let g = gradient.grad();
            let e = error.matrix();
            0.5 * g.dot(&(e * g))
        };

        let state = MinimumState::new(params, error, gradient, edm, fcn.num_of_calls());

        MinimumSeed::new(state, trafo.clone())
    }

    /// Generate seed using analytical gradients from user.
    pub fn generate_with_gradient(
        fcn: &dyn FCNGradient,
        trafo: &MnUserTransformation,
        _strategy: &MnStrategy,
    ) -> MinimumSeed {
        let n = trafo.variable_parameters();
        let eps2 = trafo.precision().eps2();

        // 1. Get initial internal parameter values
        let int_values = trafo.initial_internal_values();
        let int_vec = DVector::from_vec(int_values.clone());

        // 2. Evaluate FCN at starting point (needed for EDM calculation)
        let fval = fcn.value(&trafo.transform(&int_values));
        let params = MinimumParameters::new(int_vec, fval);

        // 3. Compute analytical gradient (user-provided, with g2/gstep heuristics)
        let gradient = AnalyticalGradientCalculator::compute(fcn, trafo, &params);

        // 4. Build V₀ = diag(1/g2_i), fallback to 1.0 for non-positive g2
        let mut v0 = DMatrix::zeros(n, n);
        for i in 0..n {
            let g2i = gradient.g2()[i];
            v0[(i, i)] = if g2i > eps2 {
                1.0 / g2i
            } else {
                1.0
            };
        }

        let dcovar = 1.0; // approximate: initial V is rough
        let error = MinimumError::new(v0, dcovar);

        // 5. EDM = 0.5 * g^T * V * g
        let edm = {
            let g = gradient.grad();
            let e = error.matrix();
            0.5 * g.dot(&(e * g))
        };

        // Note: no MnFcn call counter here since analytical gradient doesn't eval FCN
        // We'll use state with nfcn=1 (for the initial FCN eval only)
        let state = MinimumState::new(params, error, gradient, edm, 1);

        MinimumSeed::new(state, trafo.clone())
    }

    pub fn call_with_analytical_gradient_calculator(
        fcn: &dyn FCNGradient,
        trafo: &MnUserTransformation,
        strategy: &MnStrategy,
    ) -> MinimumSeed {
        Self::generate_with_gradient(fcn, trafo, strategy)
    }
}
