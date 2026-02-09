//! minuit2 — Pure Rust port of CERN Minuit2 parameter optimization engine.
//!
//! # Quick Start
//!
//! ```
//! use minuit2::MnSimplex;
//!
//! let result = MnSimplex::new()
//!     .add("x", 0.0, 0.1)
//!     .add("y", 0.0, 0.1)
//!     .minimize(&|p: &[f64]| {
//!         (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2)
//!     });
//!
//! println!("{result}");
//! ```

pub mod application;
pub mod fcn;
pub mod gradient;
pub mod minimum;
pub mod mn_fcn;
pub mod parameter;
pub mod precision;
pub mod print;
pub mod simplex;
pub mod strategy;
pub mod transform;
pub mod user_covariance;
pub mod user_parameter_state;
pub mod user_parameters;
pub mod user_transformation;

// Re-exports for convenience
pub use fcn::{FCN, FCNGradient};
pub use minimum::FunctionMinimum;
pub use parameter::MinuitParameter;
pub use precision::MnMachinePrecision;
pub use simplex::MnSimplex;
pub use strategy::MnStrategy;
pub use user_covariance::MnUserCovariance;
pub use user_parameter_state::MnUserParameterState;
pub use user_parameters::MnUserParameters;
pub use user_transformation::MnUserTransformation;
