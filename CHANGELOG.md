# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-02-09

### Added
- **MnHesse**: Full Hessian computation for accurate parameter errors and correlations
  - Gradient-based computation using numerical differentiation
  - Covariance matrix analysis and positive-definiteness handling
- **MnMinos**: Asymmetric profile-likelihood errors via iterative crossing-point search
  - One-sided and two-sided error computation
  - Robust handling of parameter boundaries
- **MnScan**: 1D parameter scans with minimum tracking
  - Automatic range detection for scan parameters
  - Tracks function values and identifies minima
- **MnContours**: 2D confidence contour computation
  - Contour error representation with asymmetric components
  - Contour tracing via iterative crossing-point search
- **Global Correlation Coefficients**: Measure parameter correlations from covariance matrix
  - Identifies weakly-constrained parameters
  - Returns invalid state for singular covariance matrices
- **Covariance Squeeze**: Remove fixed parameters from covariance matrix
  - Maintains proper dimensions after fixing parameters
  - Preserves error structure of remaining parameters
- **FunctionMinimum.set_user_state()**: Allow Hesse to inject covariance into minimization results
- 76 total tests covering all error analysis tools

### Changed
- Improved error analysis workflow integration with base minimizers

## [0.2.0] - 2026-02-08

### Added
- **MnMigrad**: Variable-metric (BFGS) minimizer with line search
  - DFP/BFGS update strategies with automatic metric selection
  - Initial gradient heuristic for improved convergence
  - EDM-based convergence detection with Minuit heritage compatibility
- **Integration tests**: 8 Migrad integration tests covering quadratic bowls, Rosenbrock, and bounded parameters
- **Numerical gradient**: 2-point central difference for automatic differentiation

## [0.1.0] - 2026-02-08

### Added
- **Core types**: Parameter, Covariance, Gradient, Precision
- **User-facing API**: MinuitParameter, MnUserTransformation, MnUserCovariance
- **Transform chains**: Unbounded, bounded (sin, sqrt_low, sqrt_up)
- **MnSimplex**: Nelder-Mead variant with rho-extrapolation
  - Reflection, expansion, contraction, and shrink operations
  - Convergence detection with current/previous EDM check
  - 6 integration tests covering minimization scenarios
- Initial setup: 42 unit tests

