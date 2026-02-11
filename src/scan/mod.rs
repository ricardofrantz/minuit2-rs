//! MnScan / MnParameterScan: 1D parameter scan.
//!
//! Replaces MnParameterScan.h/.cxx and MnScan.h. Evaluates the function
//! along one parameter direction, keeping others at their minimum values.

use crate::fcn::FCN;
use crate::minimum::FunctionMinimum;
use crate::user_parameters::MnUserParameters;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Low-level 1D parameter scan.
pub struct MnParameterScan<'a, F: FCN + ?Sized> {
    fcn: &'a F,
    params: MnUserParameters,
    fval: f64,
}

impl<'a, F: FCN + ?Sized> MnParameterScan<'a, F> {
    pub fn new(fcn: &'a F, params: MnUserParameters, fval: f64) -> Self {
        Self { fcn, params, fval }
    }

    /// Scan parameter `par` over `nsteps` points between `low` and `high`.
    ///
    /// If `low == high == 0.0`, auto-range to +/- 2*error.
    /// Returns `Vec<(parameter_value, function_value)>`.
    /// Updates internal fval and param value if a better point is found.
    pub fn scan(&mut self, par: usize, nsteps: usize, low: f64, high: f64) -> Vec<(f64, f64)> {
        self.scan_serial(par, nsteps, low, high)
    }

    /// Serial implementation of 1D scan.
    pub fn scan_serial(
        &mut self,
        par: usize,
        nsteps: usize,
        low: f64,
        high: f64,
    ) -> Vec<(f64, f64)> {
        let (nsteps, low, high, values) = self.setup_scan(par, nsteps, low, high);
        let step = (high - low) / nsteps as f64;
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

    /// Parallel implementation of 1D scan (requires `parallel` feature).
    #[cfg(feature = "parallel")]
    pub fn scan_parallel(
        &mut self,
        par: usize,
        nsteps: usize,
        low: f64,
        high: f64,
    ) -> Vec<(f64, f64)>
    where
        F: Sync,
    {
        let (nsteps, low, high, values) = self.setup_scan(par, nsteps, low, high);
        let step = (high - low) / nsteps as f64;

        let result: Vec<(f64, f64)> = (0..=nsteps)
            .into_par_iter()
            .map(|i| {
                let x = low + i as f64 * step;
                let mut pars = values.clone();
                pars[par] = x;
                (x, self.fcn.value(&pars))
            })
            .collect();

        if let Some((x_best, f_best)) = result.iter().copied().min_by(|a, b| a.1.total_cmp(&b.1))
            && f_best < self.fval
        {
            self.fval = f_best;
            self.params.set_value(par, x_best);
        }

        result
    }

    fn setup_scan(
        &self,
        par: usize,
        nsteps: usize,
        low: f64,
        high: f64,
    ) -> (usize, f64, f64, Vec<f64>) {
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

        // Build parameter vector at minimum
        let nparams = self.params.len();
        let values: Vec<f64> = (0..nparams)
            .map(|i| self.params.trafo().parameter(i).value())
            .collect();

        (nsteps, low, high, values)
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
pub struct MnScan<'a, F: FCN + ?Sized> {
    fcn: &'a F,
    minimum: &'a FunctionMinimum,
}

impl<'a, F: FCN + ?Sized> MnScan<'a, F> {
    /// Create a new high-level scan from a minimization result.
    pub fn new(fcn: &'a F, minimum: &'a FunctionMinimum) -> Self {
        Self { fcn, minimum }
    }

    /// Scan parameter `par` over `nsteps` points.
    ///
    /// If `low == high == 0.0`, auto-range to +/- 2*error.
    pub fn scan(&self, par: usize, nsteps: usize, low: f64, high: f64) -> Vec<(f64, f64)> {
        self.scan_serial(par, nsteps, low, high)
    }

    /// Serial scan implementation.
    pub fn scan_serial(&self, par: usize, nsteps: usize, low: f64, high: f64) -> Vec<(f64, f64)> {
        let mut scanner =
            MnParameterScan::new(self.fcn, self.build_user_parameters(), self.minimum.fval());
        scanner.scan_serial(par, nsteps, low, high)
    }

    /// Parallel scan implementation (requires `parallel` feature).
    #[cfg(feature = "parallel")]
    pub fn scan_parallel(&self, par: usize, nsteps: usize, low: f64, high: f64) -> Vec<(f64, f64)>
    where
        F: Sync,
    {
        let mut scanner =
            MnParameterScan::new(self.fcn, self.build_user_parameters(), self.minimum.fval());
        scanner.scan_parallel(par, nsteps, low, high)
    }

    fn build_user_parameters(&self) -> MnUserParameters {
        // Build MnUserParameters from the minimum
        let user_state = self.minimum.user_state();
        let nparams = user_state.len();

        let mut params = MnUserParameters::new();
        for i in 0..nparams {
            let p = user_state.parameter(i);
            if p.has_limits() {
                params.add_limited(
                    p.name(),
                    p.value(),
                    p.error(),
                    p.lower_limit(),
                    p.upper_limit(),
                );
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

        params
    }
}
