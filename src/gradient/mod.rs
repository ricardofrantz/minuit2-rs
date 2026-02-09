//! Gradient calculators.
//!
//! The `GradientCalculator` trait defines the interface. Concrete impls:
//! - `InitialGradientCalculator`: computes a first gradient estimate from step sizes

pub mod initial;

pub use initial::InitialGradientCalculator;

use crate::minimum::gradient::FunctionGradient;
use crate::minimum::parameters::MinimumParameters;
use crate::mn_fcn::MnFcn;
use crate::user_transformation::MnUserTransformation;

/// Trait for computing function gradients.
pub trait GradientCalculator {
    fn compute(
        &self,
        fcn: &MnFcn,
        params: &MinimumParameters,
        trafo: &MnUserTransformation,
    ) -> FunctionGradient;
}
