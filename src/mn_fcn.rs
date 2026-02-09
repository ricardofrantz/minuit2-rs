//! Call-counting FCN wrapper that operates in internal parameter space.
//!
//! Replaces MnFcn.h/.cxx + MnUserFcn.h/.cxx. Takes internal parameter
//! vectors, transforms them to external space via MnUserTransformation,
//! and calls the user's FCN. Counts every call.

use std::cell::Cell;

use crate::fcn::FCN;
use crate::user_transformation::MnUserTransformation;

/// FCN wrapper that counts calls and operates in internal parameter space.
pub struct MnFcn<'a> {
    fcn: &'a dyn FCN,
    trafo: &'a MnUserTransformation,
    num_calls: Cell<usize>,
}

impl<'a> MnFcn<'a> {
    /// Create a new MnFcn wrapper.
    pub fn new(fcn: &'a dyn FCN, trafo: &'a MnUserTransformation) -> Self {
        Self {
            fcn,
            trafo,
            num_calls: Cell::new(0),
        }
    }

    /// Evaluate the function given internal-space parameters.
    /// Transforms to external space, then calls the user's FCN.
    pub fn call(&self, internal: &[f64]) -> f64 {
        self.num_calls.set(self.num_calls.get() + 1);
        let external = self.trafo.transform(internal);
        self.fcn.value(&external)
    }

    /// Get the total number of function calls made.
    pub fn num_of_calls(&self) -> usize {
        self.num_calls.get()
    }

    /// Get the error definition from the user's FCN.
    pub fn error_def(&self) -> f64 {
        self.fcn.error_def()
    }

    /// Get the Up value (error definition).
    pub fn up(&self) -> f64 {
        self.fcn.error_def()
    }
}
