# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.3.0] - 2026-02-09

### Added

- `MnHesse` — Full Hessian computation for accurate parameter errors and covariance
- `MnMinos` — Profile-likelihood asymmetric confidence intervals
- `MnScan` — 1D parameter scans with minimum tracking
- `MnContours` — 2D confidence contour computation
- Global correlation coefficients and covariance squeeze utilities
- 76 tests (48 unit + 8 Migrad + 6 Simplex + 4 Hesse + 3 Minos + 3 Scan + 2 Contours + 2 doctests)

## [0.2.0] - 2026-02-09

### Added

- `MnMigrad` — quasi-Newton minimizer with DFP/BFGS inverse Hessian update
- Quadratic interpolation (`MnParabola`, `MnParabolaPoint`)
- Parabolic line search (`mn_linesearch`)
- Positive-definite matrix correction (`make_pos_def`)
- Two-point central difference gradient (`Numerical2PGradientCalculator`)
- Full Migrad implementation: seed, builder, minimizer, public API
- 58 tests (42 unit + 8 Migrad + 6 Simplex + 2 doctests)

## [0.1.0] - 2026-02-09

### Added

- `FCN` trait with blanket impl for closures
- Parameter management: free, bounded, fixed, constant
- Sin/sqrt parameter transforms for bounded optimization
- `MnSimplex` minimizer (Minuit variant of Nelder-Mead)
- `FunctionMinimum` result type with Display impl
- `MnStrategy` (low/medium/high optimization presets)
- `nalgebra` for all linear algebra

[Unreleased]: https://github.com/ricardofrantz/minuit2-rs/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/ricardofrantz/minuit2-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ricardofrantz/minuit2-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ricardofrantz/minuit2-rs/commits/v0.1.0
