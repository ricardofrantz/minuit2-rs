# minuit2 Documentation

Pure Rust port of CERN Minuit2 — the standard parameter optimization engine for high-energy physics.

## Table of Contents

- [Quick Start](#quick-start)
- [Minimizers](#minimizers)
  - [MnMigrad (Variable Metric)](#mnmigrad)
  - [MnSimplex (Derivative-Free)](#mnsimplex)
- [Analytical Gradients](#analytical-gradients)
- [Error Analysis](#error-analysis)
  - [MnHesse (Exact Covariance)](#mnhesse)
  - [MnMinos (Asymmetric Errors)](#mnminos)
- [Python Bindings (iminuit-style)](#python-bindings)
- [Parallel Processing](#parallel-processing)
- [Numerical Stability & Robustness](#numerical-stability--robustness)
- [Algorithm Details](#algorithm-details)

---

## Quick Start

```rust
use minuit2::MnMigrad;

let result = MnMigrad::new()
    .add("x", 0.0, 0.1)
    .add("y", 0.0, 0.1)
    .minimize(&|p: &[f64]| (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2));

if result.is_valid() {
    println!("Minimum found at: {:?}", result.params());
}
```

---

## Minimizers

### MnMigrad

**The primary workhorse.** Uses a quasi-Newton method with the Davidon-Fletcher-Powell (DFP) update.
- **Convergence:** Quadratic near the minimum.
- **Output:** Approximate covariance matrix (improved by Hesse).
- **Use case:** Smooth functions, chi-square fits, maximum likelihood.

### MnSimplex

**Derivative-free.** Uses the Nelder-Mead simplex algorithm.
- **Robustness:** Very high. Can navigate non-smooth or noisy landscapes.
- **Performance:** Slower than Migrad for smooth functions.
- **Use case:** Rugged landscapes (e.g., Goldstein-Price), noisy data, or as a pre-minimizer.

---

## Analytical Gradients

By default, `MnMigrad` uses 2-point numerical differentiation. For performance-critical or high-dimensional problems, implement `FCNGradient` to provide analytical derivatives.

```rust
use minuit2::{FCN, FCNGradient, MnMigrad};

struct MyModel;

impl FCN for MyModel {
    fn value(&self, par: &[f64]) -> f64 {
        par.iter().map(|x| x * x).sum()
    }
}

impl FCNGradient for MyModel {
    fn gradient(&self, par: &[f64]) -> Vec<f64> {
        // Return gradient vector: [df/dx0, df/dx1, ...]
        par.iter().map(|x| 2.0 * x).collect()
    }
}

let result = MnMigrad::new()
    .add("x", 1.0, 0.1)
    .minimize_grad(&MyModel);
```

**Benefits:**
- Reduces FCN calls significantly.
- More robust in steep valleys (like Rosenbrock).
- Avoids precision issues with numerical step sizes.

---

## Python Bindings

Enabled with the `python` feature. Provides an `iminuit`-compatible API in Rust via PyO3.

**Installation:**
Typically built using `maturin`. The module name is exposed as `minuit2` (or `_minuit2` depending on build config).

```python
from minuit2 import Minuit

def fcn(x, y):
    return (x - 1)**2 + (y - 2)**2

m = Minuit(fcn, x=0, y=0)
m.migrad()
print(m.values) # {'x': 1.0, 'y': 2.0}
print(m.errors) # {'x': 0.1, 'y': 0.1}
```

---

## Parallel Processing

Enabled with the `parallel` feature. Uses `rayon` to parallelize 1D parameter scans (`MnScan`).

```rust
// Parallel scan across a parameter range
let scan = MnScan::new(&fcn, &result);
// Automatically uses all available cores with rayon
let points = scan.scan(0, 100, -5.0, 5.0);
```

---

## Numerical Stability & Robustness

`minuit2-rs` implements several safety layers to ensure reliability in scientific workflows:

1.  **NaN/Inf Resilience:** If your FCN returns `NaN` or `Inf`, the optimizer treats it as a huge penalty (returning a value like `1e30`). This prevents the linear algebra core from collapsing and allows the minimizer to "back out" of illegal regions.
2.  **Positive-Definite Correction:** If the covariance matrix becomes non-positive-definite (due to negative curvature or precision loss), `minuit2-rs` automatically applies an eigenvalue shift to restore stability.
3.  **Safe Floating-Point Sorting:** All internal sorting (in LineSearch and Minos) uses `NaN`-safe comparisons to prevent panics during extreme numerical instability.

**Validation:** The library is stress-tested against the **Goldstein-Price** function and high-dimensional (50D) quadratic bowls to ensure it handles rugged and large-scale problems correctly.

---

## Algorithm Details

### Davidon-Fletcher-Powell (DFP)
The core of Migrad. It maintains an estimate of the inverse Hessian $V$. At each step:
1. Compute Newton step: $\delta x = -V \cdot \nabla f$
2. Update $V$ using the difference in parameters and gradients:
   $$V_{new} = V + \frac{\delta x \delta x^T}{\delta x^T \delta g} - \frac{V \delta g \delta g^T V}{\delta g^T V \delta g}$$

### Parameter Transformations
Bounded parameters are optimized in an unbounded internal space using the following transforms:
- **Both Bounds:** $\sin(x)$
- **Lower Only:** $\sqrt{x^2 + 1} - 1$
- **Upper Only:** $1 - \sqrt{x^2 + 1}$

This ensures the optimizer never "steps out" of user-defined boundaries while maintaining a smooth derivative landscape.

---

## Benchmark Results

| Function | Minimizer | Dim | NFCN | Valid |
|----------|-----------|-----|------|-------|
| Rosenbrock | Migrad | 2 | ~40 | Yes |
| Quadratic | Migrad | 50 | ~250 | Yes |
| Goldstein-Price | Simplex | 2 | ~90 | Yes |
| Gaussian Fit | Migrad | 3 | ~60 | Yes |
