//! MnScan / MnParameterScan: 1D parameter scan.
//!
//! Replaces MnParameterScan.h/.cxx and MnScan.h. Evaluates the function
//! along one parameter direction, keeping others at their minimum values.

use crate::fcn::FCN;
use crate::minimum::FunctionMinimum;
use crate::user_parameters::MnUserParameters;

/// Low-level 1D parameter scan.
pub struct MnParameterScan<'a> {
    fcn: &'a dyn FCN,
    params: MnUserParameters,
    fval: f64,
}

impl<'a> MnParameterScan<'a> {
    pub fn new(fcn: &'a dyn FCN, params: MnUserParameters, fval: f64) -> Self {
        Self { fcn, params, fval }
    }

    /// Scan parameter `par` over `nsteps` points between `low` and `high`.
    ///
    /// If `low == high == 0.0`, auto-range to +/- 2*error.
    /// Returns `Vec<(parameter_value, function_value)>`.
    /// Updates internal fval and param value if a better point is found.
    pub fn scan(&mut self, par: usize, nsteps: usize, low: f64, high: f64) -> Vec<(f64, f64)> {
        let nsteps = nsteps.clamp(2, 101);
        let p = self.params.trafo().parameter(par);
        let val = p.value();
        let err = p.error();

        let (low, high) = if (low - high).abs() < 1e-15 {
            // Auto-range: +/- 2*error
            let lo = val - 2.0 * err;
            let hi = val + 2.0 * err;

            // Clamp to limits
            let lo = if p.has_lower_limit() {
                lo.max(p.lower_limit())
            } else {
                lo
            };
            let hi = if p.has_upper_limit() {
                hi.min(p.upper_limit())
            } else {
                hi
            };
            (lo, hi)
        } else {
            let lo = if p.has_lower_limit() {
                low.max(p.lower_limit())
            } else {
                low
            };
            let hi = if p.has_upper_limit() {
                high.min(p.upper_limit())
            } else {
                high
            };
            (lo, hi)
        };

        let step = (high - low) / nsteps as f64;

        // Build parameter vector at minimum
        let nparams = self.params.len();
        let values: Vec<f64> = (0..nparams)
            .map(|i| self.params.trafo().parameter(i).value())
            .collect();

        let mut result = Vec::with_capacity(nsteps + 1);

        for i in 0..=nsteps {
            let x = low + i as f64 * step;
            let mut pars = values.clone();
            pars[par] = x;
            let f = self.fcn.value(&pars);
            result.push((x, f));

            // Track minimum
            if f < self.fval {
                self.fval = f;
                self.params.set_value(par, x);
            }
        }

        result
    }

    /// Current best function value (may have been updated by scan).
    pub fn fval(&self) -> f64 {
        self.fval
    }

    /// Current parameters (may have been updated by scan).
    pub fn params(&self) -> &MnUserParameters {
        &self.params
    }
}

/// High-level scan builder working with a FunctionMinimum.
pub struct MnScan<'a> {
    fcn: &'a dyn FCN,
    minimum: &'a FunctionMinimum,
}

impl<'a> MnScan<'a> {
    /// Create a new high-level scan from a minimization result.
    pub fn new(fcn: &'a dyn FCN, minimum: &'a FunctionMinimum) -> Self {
        Self { fcn, minimum }
    }

    /// Scan parameter `par` over `nsteps` points.
    ///
    /// If `low == high == 0.0`, auto-range to +/- 2*error.
    pub fn scan(&self, par: usize, nsteps: usize, low: f64, high: f64) -> Vec<(f64, f64)> {
        // Build MnUserParameters from the minimum
        let user_state = self.minimum.user_state();
        let nparams = user_state.len();

        let mut params = MnUserParameters::new();
        for i in 0..nparams {
            let p = user_state.parameter(i);
            if p.has_limits() {
                params.add_limited(p.name(), p.value(), p.error(), p.lower_limit(), p.upper_limit());
            } else if p.has_lower_limit() {
                params.add_lower_limited(p.name(), p.value(), p.error(), p.lower_limit());
            } else if p.has_upper_limit() {
                params.add_upper_limited(p.name(), p.value(), p.error(), p.upper_limit());
            } else if p.is_const() {
                params.add_const(p.name(), p.value());
            } else {
                params.add(p.name(), p.value(), p.error());
            }
        }

        let mut scanner = MnParameterScan::new(self.fcn, params, self.minimum.fval());
        scanner.scan(par, nsteps, low, high)
    }
}
