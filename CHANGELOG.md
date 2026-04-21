# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.4.2] - 2026-04-21

Security hardening and supply-chain release. No runtime API or behavior
changes; the Rust library surface is identical to 0.4.1.

### Security

- Upgraded `pyo3` 0.28.0 → 0.28.3. The 0.28.0 lockfile entry was yanked on
  crates.io and carried RUSTSEC-2026-0013 (type confusion in
  `#[pyclass(extends=<native>)]` with the `abi3` feature on Python 3.12+).
  The Python bindings in this crate do not use `extends=`, so the library
  was not materially exposed, but the yanked version was replaced anyway.
- Hardened all four GitHub Actions workflows against shell injection:
  every `${{ inputs.* }}` and `${{ github.event.inputs.* }}` used in a
  `run:` block is now routed through a step-level `env:` map so user
  input lands in bash as data, never as program source. Numeric inputs
  are validated at the top of the step.
- Hardened the release workflow against awk code injection through a
  maliciously-named tag. `VERSION` is now passed to awk via `-v` (data
  binding) and matched with `index()` (literal prefix), not spliced into
  the awk program.
- Pinned every third-party GitHub Action by commit SHA across `ci.yml`,
  `release.yml`, `scientific-demo.yml`, and `scientific-demo-scheduled.yml`
  (8 distinct actions). Floating refs like `@v4` and `@stable` can be
  force-moved upstream; SHA pins close that supply-chain vector.
- Added a `cargo-deny` CI gate alongside the existing `cargo-audit`
  check. `deny.toml` enforces a permissive license allowlist, denies
  unknown registries and git sources, and gates the advisory database
  against the entire dependency graph.
- Split scientific-demo workflows so the benchmark job runs with
  `permissions: contents: read`. Only the follow-up `commit-refresh` /
  `commit-artifacts` job carries `contents: write`, and only when gated
  on `master` (scheduled) or the `auto_commit` input (manual).
- Release job now rejects any tag whose commit is not an ancestor of
  `origin/master`: a release cannot ship from an unreviewed branch.

### Data

- Added `examples/data/SHA256SUMS` pinning the SHA-256 of every scientific
  reference data file to the blob tracked in git.
- Rewrote `scripts/fetch_scientific_demo_data.sh` to verify content
  against the manifest, stage downloads to a sibling tempfile, and only
  move into place after a successful checksum match. Restored TLS
  verification on every download (no more `curl -k`); `--proto '=https'`
  refuses any non-HTTPS scheme including on redirects. Any mismatch or
  missing manifest entry aborts with a clear diagnostic.
- Refreshed the NOAA CO₂ reference snapshot to the current upstream
  (additive February 2026 row and small retroactive uncertainty
  revisions, consistent with NOAA post-analysis practice).

### CI/CD

- Removed Miri CI job (nightly toolchain incompatibility with `quote`
  crate build script).

## [0.4.1] - 2026-02-12

### Added

- ROOT v6-36-08 verification harness with parity baseline for cross-validation
- Executed-surface gate with expanded verification workloads
- Scientific demonstration examples (Rosenbrock, Himmelbayas, Rastrigin, sphere functions)
- Fair solver benchmark workflow with automated artifact refresh

### Changed

- Internal refactoring: tightened executed-surface mapping and reduced false positives
- Enhanced `MnMinimize` internal logic and state handling
- Improved `MnScan`, `MnContours`, and `MnMinos` internal implementations
- Expanded test coverage (93 tests, up from 92)

### Fixed

- Security: upgraded PyO3 0.23 → 0.28 to resolve RUSTSEC-2025-0020
- Resolved all clippy warnings for clean CI lint gate
- Corrected two API errors in README examples
- Applied cargo fmt across entire codebase

### CI/CD

- Added scheduled scientific benchmark workflow with auto-commit to master
- Removed main branch alias, targeting master only
- Finalized examples cleanup for cleaner CI workflow

### Docs

- Expanded README with comprehensive user guide integrating DOC.md content
- Added verification snapshot and developer guide sections

## [0.4.0] - 2026-02-09

### Added

- Analytical gradient support via `AnalyticalGradientCalculator` trait — users can provide exact gradients for faster convergence
- Parallel parameter scans with rayon (opt-in via `parallel` feature) — simultaneous evaluation across scan ranges
- Python bindings via PyO3 (opt-in via `python` feature) — call minuit2 minimizers from Python
- `MnMinimize` combined minimizer — automatic Simplex → Migrad fallback strategy
- Criterion benchmarks for performance regression tracking
- Robustness test suite — NaN/Inf resilience, boundary edge cases, high-dimensional stress tests

### Changed

- Enhanced panic safety in core minimizers with comprehensive edge-case handling
- CI/CD improvements: security audit, code coverage, benchmark jobs

### Docs

- Streamlined README and DOC.md for v0.4.0 features
- Added error analysis examples
- CHANGELOG.md now used for GitHub release notes

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

[Unreleased]: https://github.com/ricardofrantz/minuit2-rs/compare/v0.4.2...HEAD
[0.4.2]: https://github.com/ricardofrantz/minuit2-rs/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/ricardofrantz/minuit2-rs/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/ricardofrantz/minuit2-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/ricardofrantz/minuit2-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ricardofrantz/minuit2-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ricardofrantz/minuit2-rs/commits/v0.1.0
