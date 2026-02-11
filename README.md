# `minuit2-rs` — Pure Rust Port of Standalone Minuit2

Minuit2 is CERN's parameter optimization engine, the standard in high-energy physics since 1975.  
This crate is a **pure Rust port of ROOT Minuit2**, specifically the `math/minuit2` subsystem from:
- `https://github.com/root-project/root/tree/master/math/minuit2`

Target compatibility baseline for verification work:
- ROOT release tag: `v6-36-08`
- Scope: **Minuit2 only** (`math/minuit2`), not the broader ROOT framework layers.

Compatibility snapshot (2026-02-11, differential harness):
- pass: `4`
- warn: `2`
- fail: `0`
- Details: `reports/verification/diff_summary.md`
- Claim scorecard: `reports/verification/scorecard.md`
- Legacy C++ executed-surface coverage: `reports/verification/reference_coverage/summary.md`

One-command reproducible verification (ROOT `v6-36-08` baseline):

```bash
scripts/run_full_verification.sh v6-36-08
```

## Features

- **Pure Rust.** No C++ toolchain, no unsafe blocks, zero-cost abstractions.
- **Robust Algorithms.** Migrad (Variable Metric), Simplex (Nelder-Mead), Hesse (Exact Errors), Minos (Asymmetric Errors).
- **Analytical Gradients.** Support for user-provided gradients for faster convergence.
- **Python Bindings.** High-performance PyO3 bindings for integration with Python workflows.
- **Parallel Processing.** Optional `rayon` support for parallel parameter scans.
- **Numerical Stability.** Resilience against `NaN` and `Infinity` with automatic recovery.

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

### Analytical Gradients

Use analytical gradients to significantly reduce the number of function calls:

```rust
use minuit2::{FCN, FCNGradient, MnMigrad};

struct MyFcn;
impl FCN for MyFcn {
    fn value(&self, p: &[f64]) -> f64 { p[0]*p[0] + p[1]*p[1] }
}
impl FCNGradient for MyFcn {
    fn gradient(&self, p: &[f64]) -> Vec<f64> { vec![2.0*p[0], 2.0*p[1]] }
}

let result = MnMigrad::new()
    .add("x", 1.0, 0.1)
    .add("y", 1.0, 0.1)
    .minimize_grad(&MyFcn);
```

### Feature Flags

- `python`: Enables PyO3 bindings (`Minuit` class).
- `parallel`: Enables `rayon` support for `MnScan::scan_parallel`.

```toml
[dependencies]
minuit2 = { version = "0.3", features = ["python", "parallel"] }
```

## Current Status

| Minimizer | Status | Description |
|-----------|--------|-------------|
| **MnMigrad** | Done | Quasi-Newton (DFP) — recommended for smooth functions |
| **MnSimplex** | Done | Nelder-Mead (Minuit variant) — derivative-free |
| **MnHesse** | Done | Full Hessian calculation for accurate errors |
| **MnMinos** | Done | Asymmetric error estimation |
| **MnScan** | Done | 1D parameter scans (`scan_parallel` available with `parallel` feature) |
| **MnContours** | Done | 2D confidence contours |

## Robustness & Security

`minuit2-rs` is built for reliability in scientific computing:
- **Zero Unsafe.** The entire codebase is written in 100% safe Rust.
- **Numerical Resilience.** Gracefully handles `NaN` and `Inf` by treating them as high-value penalties, preventing optimizer crashes.
- **Stress Tested.** Validated against 50-parameter problems and rugged landscapes like the **Goldstein-Price** function.

## Upstream Source

[ROOT Minuit2 (`math/minuit2`)](https://github.com/root-project/root/tree/master/math/minuit2).
This port replaces custom C++ linear algebra with `nalgebra` and manual memory management with Rust's ownership model.

## License

MIT OR Apache-2.0
