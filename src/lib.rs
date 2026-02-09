//! minuit2 — Pure Rust port of CERN Minuit2 parameter optimization engine.
//!
//! # Quick Start — Migrad (recommended)
//!
//! ```
//! use minuit2::MnMigrad;
//!
//! let result = MnMigrad::new()
//!     .add("x", 0.0, 0.1)
//!     .add("y", 0.0, 0.1)
//!     .minimize(&|p: &[f64]| {
//!         (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2)
//!     });
//!
//! println!("{result}");
//! ```
//!
//! # Simplex (derivative-free)
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
pub mod contours;
pub mod covariance_squeeze;
pub mod fcn;
pub mod global_cc;
pub mod gradient;
pub mod hesse;
pub mod linesearch;
pub mod migrad;
pub mod minimize;
pub mod minimum;
pub mod minos;
pub mod mn_fcn;
pub mod parabola;
pub mod parameter;
pub mod posdef;
pub mod precision;
pub mod print;
pub mod scan;
pub mod simplex;
pub mod strategy;
pub mod transform;
pub mod user_covariance;
pub mod user_parameter_state;
pub mod user_parameters;
pub mod user_transformation;

// Re-exports for convenience
pub use contours::MnContours;
pub use fcn::{FCN, FCNGradient};
pub use hesse::MnHesse;
pub use migrad::MnMigrad;
pub use minimize::MnMinimize;
pub use minimum::FunctionMinimum;
pub use minos::MnMinos;
pub use parameter::MinuitParameter;
pub use precision::MnMachinePrecision;
pub use scan::MnScan;
pub use simplex::MnSimplex;
pub use strategy::MnStrategy;
pub use user_covariance::MnUserCovariance;
pub use user_parameter_state::MnUserParameterState;
pub use user_parameters::MnUserParameters;
pub use user_transformation::MnUserTransformation;
