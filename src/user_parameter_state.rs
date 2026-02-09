//! Complete parameter state: parameters + function value + EDM + nfcn + covariance.
//!
//! Replaces MnUserParameterState.h/.cxx. This is the state object returned to
//! the user after minimization, containing fitted values, errors, and
//! optionally the covariance matrix.

use crate::parameter::MinuitParameter;
use crate::user_covariance::MnUserCovariance;
use crate::user_parameters::MnUserParameters;

#[derive(Debug, Clone)]
pub struct MnUserParameterState {
    params: MnUserParameters,
    covariance: Option<MnUserCovariance>,
    global_cc: Option<Vec<f64>>,
    fval: f64,
    edm: f64,
    nfcn: usize,
    is_valid: bool,
    covariance_valid: bool,
}

impl MnUserParameterState {
    /// Create initial state from user parameters (pre-minimization).
    pub fn new(params: MnUserParameters) -> Self {
        Self {
            params,
            covariance: None,
            global_cc: None,
            fval: 0.0,
            edm: 0.0,
            nfcn: 0,
            is_valid: false,
            covariance_valid: false,
        }
    }

    pub fn params(&self) -> &MnUserParameters {
        &self.params
    }

    pub fn params_mut(&mut self) -> &mut MnUserParameters {
        &mut self.params
    }

    pub fn fval(&self) -> f64 {
        self.fval
    }

    pub fn set_fval(&mut self, fval: f64) {
        self.fval = fval;
    }

    pub fn edm(&self) -> f64 {
        self.edm
    }

    pub fn set_edm(&mut self, edm: f64) {
        self.edm = edm;
    }

    pub fn nfcn(&self) -> usize {
        self.nfcn
    }

    pub fn set_nfcn(&mut self, nfcn: usize) {
        self.nfcn = nfcn;
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn set_valid(&mut self, valid: bool) {
        self.is_valid = valid;
    }

    pub fn has_covariance(&self) -> bool {
        self.covariance_valid
    }

    pub fn covariance(&self) -> Option<&MnUserCovariance> {
        self.covariance.as_ref()
    }

    pub fn set_covariance(&mut self, cov: MnUserCovariance) {
        self.covariance_valid = true;
        self.covariance = Some(cov);
    }

    pub fn global_cc(&self) -> Option<&[f64]> {
        self.global_cc.as_deref()
    }

    pub fn set_global_cc(&mut self, gcc: Vec<f64>) {
        self.global_cc = Some(gcc);
    }

    // --- Delegation to MnUserParameters ---

    pub fn add(&mut self, name: impl Into<String>, value: f64, error: f64) -> usize {
        self.params.add(name, value, error)
    }

    pub fn add_limited(&mut self, name: impl Into<String>, value: f64, error: f64, lower: f64, upper: f64) -> usize {
        self.params.add_limited(name, value, error, lower, upper)
    }

    pub fn fix(&mut self, ext: usize) {
        self.params.fix(ext);
    }

    pub fn release(&mut self, ext: usize) {
        self.params.release(ext);
    }

    pub fn set_value(&mut self, ext: usize, val: f64) {
        self.params.set_value(ext, val);
    }

    pub fn set_error(&mut self, ext: usize, err: f64) {
        self.params.set_error(ext, err);
    }

    pub fn value(&self, name: &str) -> Option<f64> {
        self.params.value(name)
    }

    pub fn error(&self, name: &str) -> Option<f64> {
        self.params.error(name)
    }

    pub fn parameter(&self, ext: usize) -> &MinuitParameter {
        self.params.trafo().parameter(ext)
    }

    pub fn variable_parameters(&self) -> usize {
        self.params.variable_parameters()
    }

    pub fn len(&self) -> usize {
        self.params.len()
    }

    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }
}
