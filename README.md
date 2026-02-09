# `minuit2-rs` — Pure Rust Port of Standalone Minuit2

Minuit2 is CERN's parameter optimization engine, the standard in high-energy physics since 1975. This project ports the [standalone extraction](https://github.com/GooFit/Minuit2) to Rust — no C++, no ROOT.

## Why

- **No Rust equivalent.** `argmin`/`ganesh` lack Minuit-specific features: Hesse error matrices, Minos asymmetric errors, contours, chi-square error propagation.
- **High impact.** HEP/experimental physics needs chi-square fits without ROOT bloat.
- **Safety + speed.** No C++ build headaches, no memory bugs, easy parallelism.

## Upstream Source

[GooFit/Minuit2](https://github.com/GooFit/Minuit2) — ~14.5k LOC, 187 C++ files (76 `.cxx` + 111 `.h`), standalone CMake build.

Of these, **~50 files are core** (the rest are ROOT adapters, Fumili, MPI, BLAS-like routines replaceable by `nalgebra`). See [INVENTORY.md](INVENTORY.md) for the full file map.

## Plan (~4-6 months solo)

| Phase | Scope | Duration | Plan |
|-------|-------|----------|------|
| 1 | Core types, traits, parameters, linalg | 2-4 wk | [plan_phase1.md](plan_phase1.md) |
| 2 | Simplex minimizer | 3-4 wk | [plan_phase2.md](plan_phase2.md) |
| 3 | Migrad (variable metric) | 4-6 wk | [plan_phase3.md](plan_phase3.md) |
| 4 | Hesse, Minos, Scan, Contours | 3-4 wk | [plan_phase4.md](plan_phase4.md) |
| 5 | CLI, benchmarks, crates.io | 2-3 wk | [plan_phase5.md](plan_phase5.md) |

## Goals

- Crate on crates.io: `minuit2-rs`
- Idiomatic Rust API (traits + builders)
- Algorithms: Migrad, Simplex, Hesse, Minos, Scan/Contours
- `nalgebra` for linalg, optional `rayon` for parallel scans
- Validated against iminuit on physics benchmarks
- Stretch: PyO3 Python bindings
