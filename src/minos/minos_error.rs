//! MinosError: paired upper/lower crossing results for one parameter.
//!
//! Replaces MinosError.h. Wraps two MnCross results (upper and lower) and
//! computes the actual parameter error in external space.

use super::cross::MnCross;

/// Asymmetric MINOS errors for a single parameter.
#[derive(Debug, Clone)]
pub struct MinosError {
    /// External parameter index.
    parameter: usize,
    /// Hesse (parabolic) error for this parameter.
    hesse_error: f64,
    /// Lower crossing result.
    lower: MnCross,
    /// Upper crossing result.
    upper: MnCross,
}

impl MinosError {
    pub fn new(parameter: usize, hesse_error: f64, lower: MnCross, upper: MnCross) -> Self {
        Self {
            parameter,
            hesse_error,
            lower,
            upper,
        }
    }

    /// The lower (negative) MINOS error.
    ///
    /// Returns: -err * (1 + lower.value) if valid, else -hesse_error.
    pub fn lower_error(&self) -> f64 {
        if self.lower.is_valid() {
            -self.hesse_error * (1.0 + self.lower.value())
        } else {
            -self.hesse_error
        }
    }

    /// The upper (positive) MINOS error.
    ///
    /// Returns: err * (1 + upper.value) if valid, else hesse_error.
    pub fn upper_error(&self) -> f64 {
        if self.upper.is_valid() {
            self.hesse_error * (1.0 + self.upper.value())
        } else {
            self.hesse_error
        }
    }

    pub fn parameter(&self) -> usize {
        self.parameter
    }

    pub fn lower(&self) -> &MnCross {
        &self.lower
    }

    pub fn upper(&self) -> &MnCross {
        &self.upper
    }

    pub fn is_valid(&self) -> bool {
        self.lower.is_valid() && self.upper.is_valid()
    }

    pub fn lower_valid(&self) -> bool {
        self.lower.is_valid()
    }

    pub fn upper_valid(&self) -> bool {
        self.upper.is_valid()
    }

    pub fn at_lower_limit(&self) -> bool {
        self.lower.at_limit()
    }

    pub fn at_upper_limit(&self) -> bool {
        self.upper.at_limit()
    }

    pub fn at_lower_max_fcn(&self) -> bool {
        self.lower.at_max_fcn()
    }

    pub fn at_upper_max_fcn(&self) -> bool {
        self.upper.at_max_fcn()
    }

    pub fn lower_new_min(&self) -> bool {
        self.lower.new_minimum()
    }

    pub fn upper_new_min(&self) -> bool {
        self.upper.new_minimum()
    }

    pub fn nfcn(&self) -> usize {
        self.lower.nfcn() + self.upper.nfcn()
    }
}
