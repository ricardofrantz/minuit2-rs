# `minuit2-rs` — Pure Rust Port of Standalone Minuit2

Minuit2 is CERN's parameter optimization engine, the standard in high-energy physics since 1975. This project ports the [standalone extraction](https://github.com/GooFit/Minuit2) to Rust — no C++, no ROOT.

## Quick Start

```rust
use minuit2::MnMigrad;

let result = MnMigrad::new()
    .add("x", 0.0, 0.1)
    .add("y", 0.0, 0.1)
    .minimize(&|p: &[f64]| {
        (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2)
    });

println!("{result}");
```

See [DOC.md](DOC.md) for full documentation.

## Why

- **No Rust equivalent.** `argmin`/`ganesh` lack Minuit-specific features: Hesse error matrices, Minos asymmetric errors, contours, chi-square error propagation.
- **High impact.** HEP/experimental physics needs chi-square fits without ROOT bloat.
- **Safety + speed.** No C++ build headaches, no memory bugs, easy parallelism.

## Current Status

| Minimizer | Status | Description |
|-----------|--------|-------------|
| **MnMigrad** | Done | Quasi-Newton (DFP) — recommended for smooth functions |
| **MnSimplex** | Done | Nelder-Mead (Minuit variant) — derivative-free |
| MnHesse | Planned | Full Hessian calculation for accurate errors |
| MnMinos | Planned | Asymmetric error estimation |
| MnContours | Planned | 2D confidence contours |

## Upstream Source

[GooFit/Minuit2](https://github.com/GooFit/Minuit2) — ~14.5k LOC, 187 C++ files (76 `.cxx` + 111 `.h`), standalone CMake build.

Of these, **~50 files are core** (the rest are ROOT adapters, Fumili, MPI, BLAS-like routines replaceable by `nalgebra`). See [INVENTORY.md](INVENTORY.md) for the full file map.

## Roadmap

| Phase | Scope | Status | Plan |
|-------|-------|--------|------|
| 1 | Core types, traits, parameters, linalg | Done | [plan_phase1.md](plan_phase1.md) |
| 2 | Simplex minimizer | Done | [plan_phase2.md](plan_phase2.md) |
| 3 | Migrad (variable metric) | Done | [plan_phase3.md](plan_phase3.md) |
| 4 | Hesse, Minos, Scan, Contours | Next | [plan_phase4.md](plan_phase4.md) |
| 5 | CLI, benchmarks, crates.io | Planned | [plan_phase5.md](plan_phase5.md) |

## Goals

- Crate on crates.io: `minuit2-rs`
- Idiomatic Rust API (traits + builders)
- Algorithms: Migrad, Simplex, Hesse, Minos, Scan/Contours
- `nalgebra` for linalg, optional `rayon` for parallel scans
- Validated against iminuit on physics benchmarks
- Stretch: PyO3 Python bindings

## Changelog

### v0.2.0 — Migrad (Variable-Metric Minimizer)

**New minimizer: `MnMigrad`** — quasi-Newton minimizer with DFP/BFGS inverse Hessian update. The workhorse algorithm for smooth function minimization. Converges quadratically near the minimum and produces an approximate covariance matrix.

**New files:**
- `src/parabola.rs` — Quadratic interpolation (`MnParabola`, `MnParabolaPoint`)
- `src/linesearch.rs` — Parabolic line search (`mn_linesearch`)
- `src/posdef.rs` — Positive-definite matrix correction (`make_pos_def`)
- `src/gradient/numerical.rs` — Two-point central difference gradient (`Numerical2PGradientCalculator`)
- `src/migrad/` — Full Migrad implementation:
  - `seed.rs` — Seed generator (FCN eval + numerical gradient + initial V₀)
  - `builder.rs` — Variable-metric iteration loop + DFP rank-2 update
  - `minimizer.rs` — Orchestrator (seed → builder → FunctionMinimum)
  - `mod.rs` — Public `MnMigrad` builder API

**Tests:** 58 total (42 unit + 8 Migrad integration + 6 Simplex integration + 2 doctests)

**API:** Same builder pattern as MnSimplex — drop-in replacement:
```rust
// Before (Simplex)
MnSimplex::new().add("x", 0.0, 0.1).minimize(&fcn)
// After (Migrad)
MnMigrad::new().add("x", 0.0, 0.1).minimize(&fcn)
```

### v0.1.0 — Core Types + Simplex

Initial release with core parameter types, transforms, and the Nelder-Mead simplex minimizer.

- `FCN` trait with blanket impl for closures
- Parameter management: free, bounded, fixed, constant
- Sin/sqrt parameter transforms for bounded optimization
- `MnSimplex` minimizer (Minuit variant of Nelder-Mead)
- `FunctionMinimum` result type with Display impl
- `MnStrategy` (low/medium/high optimization presets)
- `nalgebra` for all linear algebra (replaces 28 C++ files)
