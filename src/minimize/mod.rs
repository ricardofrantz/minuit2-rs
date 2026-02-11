//! Public Minimize minimizer API (hybrid Simplex + Migrad).
//!
//! `MnMinimize` is a combined minimizer that uses a two-phase approach:
//! 1. Runs Simplex (derivative-free) to locate the approximate minimum
//! 2. Then runs Migrad (variable-metric) from that point for precise convergence
//!
//! This hybrid approach is robust for difficult functions and has fast convergence near the minimum.
//! Uses a builder pattern to configure parameters, then call `minimize()`.

use crate::application::DEFAULT_TOLERANCE;
use crate::fcn::FCN;
use crate::migrad::MnMigrad;
use crate::minimum::FunctionMinimum;
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
            tolerance: DEFAULT_TOLERANCE,
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
    pub fn add_limited(
        mut self,
        name: impl Into<String>,
        value: f64,
        error: f64,
        lower: f64,
        upper: f64,
    ) -> Self {
        self.params.add_limited(name, value, error, lower, upper);
        self
    }

    /// Add a parameter with lower bound only.
    pub fn add_lower_limited(
        mut self,
        name: impl Into<String>,
        value: f64,
        error: f64,
        lower: f64,
    ) -> Self {
        self.params.add_lower_limited(name, value, error, lower);
        self
    }

    /// Add a parameter with upper bound only.
    pub fn add_upper_limited(
        mut self,
        name: impl Into<String>,
        value: f64,
        error: f64,
        upper: f64,
    ) -> Self {
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

    /// Set tolerance (relative to error_def). Default = 0.1.
    pub fn tolerance(mut self, tol: f64) -> Self {
        self.tolerance = tol;
        self
    }

    fn configure_simplex_from_params(
        mut simplex: MnSimplex,
        params: &MnUserParameters,
    ) -> MnSimplex {
        for param in params.params() {
            if param.is_const() {
                simplex = simplex.add_const(param.name(), param.value());
            } else if param.has_limits() {
                simplex = simplex.add_limited(
                    param.name(),
                    param.value(),
                    param.error(),
                    param.lower_limit(),
                    param.upper_limit(),
                );
            } else if param.has_lower_limit() {
                simplex = simplex.add_lower_limited(
                    param.name(),
                    param.value(),
                    param.error(),
                    param.lower_limit(),
                );
            } else if param.has_upper_limit() {
                simplex = simplex.add_upper_limited(
                    param.name(),
                    param.value(),
                    param.error(),
                    param.upper_limit(),
                );
            } else {
                simplex = simplex.add(param.name(), param.value(), param.error());
            }

            if param.is_fixed() && !param.is_const() {
                simplex = simplex.fix(param.number());
            }
        }
        simplex
    }

    fn configure_migrad_from_params(mut migrad: MnMigrad, params: &MnUserParameters) -> MnMigrad {
        for param in params.params() {
            if param.is_const() {
                migrad = migrad.add_const(param.name(), param.value());
            } else if param.has_limits() {
                migrad = migrad.add_limited(
                    param.name(),
                    param.value(),
                    param.error(),
                    param.lower_limit(),
                    param.upper_limit(),
                );
            } else if param.has_lower_limit() {
                migrad = migrad.add_lower_limited(
                    param.name(),
                    param.value(),
                    param.error(),
                    param.lower_limit(),
                );
            } else if param.has_upper_limit() {
                migrad = migrad.add_upper_limited(
                    param.name(),
                    param.value(),
                    param.error(),
                    param.upper_limit(),
                );
            } else {
                migrad = migrad.add(param.name(), param.value(), param.error());
            }

            if param.is_fixed() && !param.is_const() {
                migrad = migrad.fix(param.number());
            }
        }
        migrad
    }

    /// Run the combined minimization.
    ///
    /// ROOT Minuit2 parity:
    /// 1) Try Migrad first.
    /// 2) If Migrad fails, run Simplex with strategy 2.
    /// 3) If Simplex succeeds, run Migrad again from that point (strategy 2).
    /// 4) If second Migrad fails, return the Simplex minimum.
    pub fn minimize(&self, fcn: &dyn FCN) -> FunctionMinimum {
        let n = self.params.variable_parameters();
        let max_fcn = self.max_fcn.unwrap_or(200 + 100 * n + 5 * n * n);

        // Attempt 1: Migrad with user-selected strategy.
        let migrad = Self::configure_migrad_from_params(
            MnMigrad::new().with_strategy(self.strategy.strategy()),
            &self.params,
        )
        .max_fcn(max_fcn)
        .tolerance(self.tolerance);
        let min = migrad.minimize(fcn);

        if min.is_valid() {
            return min;
        }

        // Fallback path (ROOT CombinedMinimumBuilder): use strategy 2.
        let fallback_strategy = 2_u32;
        let simplex = Self::configure_simplex_from_params(
            MnSimplex::new().with_strategy(fallback_strategy),
            &self.params,
        )
        .max_fcn(max_fcn)
        .tolerance(self.tolerance);
        let simplex_min = simplex.minimize(fcn);

        if !simplex_min.is_valid() {
            return simplex_min;
        }

        let migrad2 = Self::configure_migrad_from_params(
            MnMigrad::new().with_strategy(fallback_strategy),
            &simplex_min.user_state().params(),
        )
        .max_fcn(max_fcn)
        .tolerance(self.tolerance);
        let min2 = migrad2.minimize(fcn);

        if min2.is_valid() { min2 } else { simplex_min }
    }
}

impl Default for MnMinimize {
    fn default() -> Self {
        Self::new()
    }
}
