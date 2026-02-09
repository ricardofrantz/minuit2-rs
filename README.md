# `minuit2-rs` — Pure Rust Port of Standalone Minuit2

Minuit2 is CERN's parameter optimization engine, the standard in high-energy physics since 1975. This project ports the [standalone extraction](https://github.com/GooFit/Minuit2) to Rust — no C++, no ROOT.

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
- `parallel`: Enables `rayon` support for parallel `MnScan`.

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
| **MnScan** | Done | 1D parameter scans (Parallel support available) |
| **MnContours** | Done | 2D confidence contours |

## Robustness & Security

`minuit2-rs` is built for reliability in scientific computing:
- **Zero Unsafe.** The entire codebase is written in 100% safe Rust.
- **Numerical Resilience.** Gracefully handles `NaN` and `Inf` by treating them as high-value penalties, preventing optimizer crashes.
- **Stress Tested.** Validated against 50-parameter problems and rugged landscapes like the **Goldstein-Price** function.

## Upstream Source

[GooFit/Minuit2](https://github.com/GooFit/Minuit2) — ~14.5k LOC, 187 C++ files.
This port replaces custom C++ linear algebra with `nalgebra` and manual memory management with Rust's ownership model.

## License

MIT OR Apache-2.0
