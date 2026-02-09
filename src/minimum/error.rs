//! Error matrix (inverse Hessian) at the minimum.
//!
//! Replaces BasicMinimumError.h/.cxx. The error matrix is the covariance
//! matrix in internal parameter space. Various status flags track how it
//! was obtained and whether it's reliable.

use nalgebra::DMatrix;

/// How the error matrix was obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorMatrixStatus {
    /// Not calculated yet.
    NotAvailable,
    /// Approximation from initial step sizes.
    ApproximateFromSteps,
    /// Forced positive-definite (may not be accurate).
    MadePositiveDefinite,
    /// Full accurate calculation.
    Accurate,
}

#[derive(Debug, Clone)]
pub struct MinimumError {
    /// Inverse Hessian matrix in internal space.
    matrix: DMatrix<f64>,
    /// The Dcovar value (distance from full covariance).
    dcovar: f64,
    /// Status of the error matrix calculation.
    status: ErrorMatrixStatus,
    /// Whether the Hessian was inverted successfully.
    hesse_failed: bool,
    /// Whether the matrix was made positive definite.
    made_pos_def: bool,
    /// Whether the inversion was valid.
    invert_failed: bool,
    /// Valid overall.
    valid: bool,
}

impl MinimumError {
    /// Create an approximate error matrix from a diagonal (step sizes squared).
    pub fn from_diagonal(n: usize) -> Self {
        Self {
            matrix: DMatrix::identity(n, n),
            dcovar: 1.0,
            status: ErrorMatrixStatus::ApproximateFromSteps,
            hesse_failed: false,
            made_pos_def: false,
            invert_failed: false,
            valid: true,
        }
    }

    /// Create from a full inverse Hessian matrix.
    pub fn new(matrix: DMatrix<f64>, dcovar: f64) -> Self {
        Self {
            matrix,
            dcovar,
            status: ErrorMatrixStatus::Accurate,
            hesse_failed: false,
            made_pos_def: false,
            invert_failed: false,
            valid: true,
        }
    }

    pub fn matrix(&self) -> &DMatrix<f64> {
        &self.matrix
    }

    pub fn dcovar(&self) -> f64 {
        self.dcovar
    }

    pub fn status(&self) -> ErrorMatrixStatus {
        self.status
    }

    pub fn set_status(&mut self, status: ErrorMatrixStatus) {
        self.status = status;
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn is_accurate(&self) -> bool {
        self.status == ErrorMatrixStatus::Accurate
    }

    pub fn is_pos_def(&self) -> bool {
        !self.made_pos_def
    }

    pub fn hesse_failed(&self) -> bool {
        self.hesse_failed
    }

    pub fn set_hesse_failed(&mut self, failed: bool) {
        self.hesse_failed = failed;
        if failed {
            self.valid = false;
        }
    }

    pub fn set_made_pos_def(&mut self, made: bool) {
        self.made_pos_def = made;
        if made {
            self.status = ErrorMatrixStatus::MadePositiveDefinite;
        }
    }

    pub fn set_invert_failed(&mut self, failed: bool) {
        self.invert_failed = failed;
        if failed {
            self.valid = false;
        }
    }

    /// Inverse of the error matrix = the Hessian itself.
    pub fn hessian(&self) -> Option<DMatrix<f64>> {
        self.matrix.clone().try_inverse()
    }
}
