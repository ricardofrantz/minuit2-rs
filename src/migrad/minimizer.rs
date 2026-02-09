//! VariableMetricMinimizer: composes seed generator + builder.
//!
//! Replaces VariableMetricMinimizer.h. Orchestrates the Migrad minimization
//! by generating the seed, then running the VariableMetricBuilder loop.

use crate::fcn::FCNGradient;
use crate::minimum::FunctionMinimum;
use crate::mn_fcn::MnFcn;
use crate::strategy::MnStrategy;
use crate::user_transformation::MnUserTransformation;
use super::builder::VariableMetricBuilder;
use super::seed::MigradSeedGenerator;

pub struct VariableMetricMinimizer;

impl VariableMetricMinimizer {
    /// Minimize using numerical gradients (central differences).
    pub fn minimize(
        fcn: &MnFcn,
        trafo: &MnUserTransformation,
        strategy: &MnStrategy,
        maxfcn: usize,
        tolerance: f64,
    ) -> FunctionMinimum {
        let up = fcn.error_def();

        // Generate seed: FCN eval + numerical gradient + V₀
        let seed = MigradSeedGenerator::generate(fcn, trafo, strategy);

        if !seed.is_valid() {
            return FunctionMinimum::new(seed, Vec::new(), up);
        }

        // EDM tolerance: F77 Minuit compatibility factor
        let edmval = tolerance * up * 0.002;

        // Run variable-metric iteration
        let states = VariableMetricBuilder::minimum(fcn, &seed, strategy, maxfcn, edmval);

        // Check outcome
        let nfcn = fcn.num_of_calls();
        if nfcn >= maxfcn {
            FunctionMinimum::with_call_limit(seed, states, up)
        } else if let Some(last) = states.last() {
            if last.edm() > 10.0 * edmval {
                FunctionMinimum::above_max_edm(seed, states, up)
            } else {
                FunctionMinimum::new(seed, states, up)
            }
        } else {
            FunctionMinimum::new(seed, states, up)
        }
    }

    /// Minimize using analytical gradients provided by the user.
    pub fn minimize_with_gradient(
        fcn: &dyn FCNGradient,
        trafo: &MnUserTransformation,
        strategy: &MnStrategy,
        maxfcn: usize,
        tolerance: f64,
    ) -> FunctionMinimum {
        let up = fcn.error_def();

        // Generate seed: FCN eval + analytical gradient + V₀
        let seed = MigradSeedGenerator::generate_with_gradient(fcn, trafo, strategy);

        if !seed.is_valid() {
            return FunctionMinimum::new(seed, Vec::new(), up);
        }

        // EDM tolerance: F77 Minuit compatibility factor
        let edmval = tolerance * up * 0.002;

        // Create a temporary MnFcn for call counting during iteration
        let mn_fcn = MnFcn::new(fcn, trafo);

        // Run variable-metric iteration with analytical gradient calculator
        let states = VariableMetricBuilder::minimum_with_gradient(
            &mn_fcn,
            fcn,
            &seed,
            strategy,
            maxfcn,
            edmval,
        );

        // Check outcome
        let nfcn = mn_fcn.num_of_calls();
        if nfcn >= maxfcn {
            FunctionMinimum::with_call_limit(seed, states, up)
        } else if let Some(last) = states.last() {
            if last.edm() > 10.0 * edmval {
                FunctionMinimum::above_max_edm(seed, states, up)
            } else {
                FunctionMinimum::new(seed, states, up)
            }
        } else {
            FunctionMinimum::new(seed, states, up)
        }
    }
}
