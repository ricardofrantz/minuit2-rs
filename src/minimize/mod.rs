//! Public Minimize minimizer API (hybrid Simplex + Migrad).
//!
//! `MnMinimize` is a combined minimizer that uses a two-phase approach:
//! 1. Runs Simplex (derivative-free) to locate the approximate minimum
//! 2. Then runs Migrad (variable-metric) from that point for precise convergence
//!
//! This hybrid approach is robust for difficult functions and has fast convergence near the minimum.
//! Uses a builder pattern to configure parameters, then call `minimize()`.

use crate::fcn::FCN;
use crate::minimum::FunctionMinimum;
use crate::migrad::MnMigrad;
use crate::simplex::MnSimplex;
use crate::strategy::MnStrategy;
use crate::user_parameters::MnUserParameters;

/// Builder for configuring and running hybrid Simplex+Migrad minimization.
pub struct MnMinimize {
    params: MnUserParameters,
    strategy: MnStrategy,
    max_fcn: Option<usize>,
    tolerance: f64,
}

impl MnMinimize {
    /// Create a new Minimize minimizer with default strategy.
    pub fn new() -> Self {
        Self {
            params: MnUserParameters::new(),
            strategy: MnStrategy::default(),
            max_fcn: None,
            tolerance: 1.0,
        }
    }

    /// Set strategy level (0=low, 1=medium, 2=high).
    pub fn with_strategy(mut self, level: u32) -> Self {
        self.strategy = MnStrategy::new(level);
        self
    }

    /// Add a free parameter.
    pub fn add(mut self, name: impl Into<String>, value: f64, error: f64) -> Self {
        self.params.add(name, value, error);
        self
    }

    /// Add a parameter with both bounds.
    pub fn add_limited(mut self, name: impl Into<String>, value: f64, error: f64, lower: f64, upper: f64) -> Self {
        self.params.add_limited(name, value, error, lower, upper);
        self
    }

    /// Add a parameter with lower bound only.
    pub fn add_lower_limited(mut self, name: impl Into<String>, value: f64, error: f64, lower: f64) -> Self {
        self.params.add_lower_limited(name, value, error, lower);
        self
    }

    /// Add a parameter with upper bound only.
    pub fn add_upper_limited(mut self, name: impl Into<String>, value: f64, error: f64, upper: f64) -> Self {
        self.params.add_upper_limited(name, value, error, upper);
        self
    }

    /// Add a constant parameter.
    pub fn add_const(mut self, name: impl Into<String>, value: f64) -> Self {
        self.params.add_const(name, value);
        self
    }

    /// Fix parameter by index.
    pub fn fix(mut self, ext: usize) -> Self {
        self.params.fix(ext);
        self
    }

    /// Set maximum number of function calls. Default = 200 + 100*n + 5*n^2.
    pub fn max_fcn(mut self, max: usize) -> Self {
        self.max_fcn = Some(max);
        self
    }

    /// Set tolerance (relative to error_def). Default = 1.0.
    pub fn tolerance(mut self, tol: f64) -> Self {
        self.tolerance = tol;
        self
    }

    /// Run the hybrid minimization (Simplex then Migrad).
    pub fn minimize(&self, fcn: &dyn FCN) -> FunctionMinimum {
        let n = self.params.variable_parameters();
        let max_fcn = self.max_fcn.unwrap_or(200 + 100 * n + 5 * n * n);

        // Phase 1: Run Simplex for global exploration
        let simplex = MnSimplex::new()
            .with_strategy(self.strategy.strategy());

        // Re-add all parameters to simplex with same configuration
        let mut simplex_builder = simplex;
        for param in self.params.params().iter() {
            if param.is_const() {
                simplex_builder = simplex_builder.add_const(param.name(), param.value());
            } else if param.has_limits() {
                simplex_builder = simplex_builder.add_limited(
                    param.name(),
                    param.value(),
                    param.error(),
                    param.lower_limit(),
                    param.upper_limit(),
                );
            } else if param.has_lower_limit() {
                simplex_builder = simplex_builder.add_lower_limited(
                    param.name(),
                    param.value(),
                    param.error(),
                    param.lower_limit(),
                );
            } else if param.has_upper_limit() {
                simplex_builder = simplex_builder.add_upper_limited(
                    param.name(),
                    param.value(),
                    param.error(),
                    param.upper_limit(),
                );
            } else {
                simplex_builder = simplex_builder.add(param.name(), param.value(), param.error());
            }

            if param.is_fixed() && !param.is_const() {
                simplex_builder = simplex_builder.fix(param.number());
            }
        }

        let simplex_builder = simplex_builder
            .max_fcn(max_fcn / 2) // Use half budget for simplex
            .tolerance(self.tolerance);

        let simplex_result = simplex_builder.minimize(fcn);

        // Phase 2: Run Migrad from simplex result
        let extracted_params = simplex_result.params();
        let user_params = simplex_result.user_state().params();

        let mut migrad = MnMigrad::new()
            .with_strategy(self.strategy.strategy());

        // Re-seed Migrad with extracted parameter values
        for (i, param) in user_params.params().iter().enumerate() {
            if param.is_const() {
                migrad = migrad.add_const(param.name(), param.value());
            } else if param.has_limits() {
                migrad = migrad.add_limited(
                    param.name(),
                    extracted_params[i],
                    param.error(),
                    param.lower_limit(),
                    param.upper_limit(),
                );
            } else if param.has_lower_limit() {
                migrad = migrad.add_lower_limited(
                    param.name(),
                    extracted_params[i],
                    param.error(),
                    param.lower_limit(),
                );
            } else if param.has_upper_limit() {
                migrad = migrad.add_upper_limited(
                    param.name(),
                    extracted_params[i],
                    param.error(),
                    param.upper_limit(),
                );
            } else {
                migrad = migrad.add(param.name(), extracted_params[i], param.error());
            }

            if param.is_fixed() && !param.is_const() {
                migrad = migrad.fix(i);
            }
        }

        let migrad = migrad
            .max_fcn(max_fcn / 2) // Use remaining half budget for migrad
            .tolerance(self.tolerance);

        migrad.minimize(fcn)
    }
}

impl Default for MnMinimize {
    fn default() -> Self {
        Self::new()
    }
}
