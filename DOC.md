# minuit2 Documentation

Pure Rust port of CERN Minuit2 — the standard parameter optimization engine for high-energy physics. This document covers all public types, algorithms, and usage patterns through v0.2.0.

## Table of Contents

- [Quick Start](#quick-start)
- [Minimizers](#minimizers)
  - [MnMigrad (recommended)](#mnmigrad)
  - [MnSimplex (derivative-free)](#mnsimplex)
- [Defining a Function (FCN)](#defining-a-function-fcn)
- [Parameters](#parameters)
  - [Free Parameters](#free-parameters)
  - [Bounded Parameters](#bounded-parameters)
  - [Fixed Parameters](#fixed-parameters)
  - [Constant Parameters](#constant-parameters)
- [Interpreting Results](#interpreting-results)
  - [FunctionMinimum](#functionminimum)
  - [User Parameter State](#user-parameter-state)
- [Convergence Control](#convergence-control)
  - [Tolerance](#tolerance)
  - [Maximum Function Calls](#maximum-function-calls)
  - [Strategy](#strategy)
- [Algorithm Details](#algorithm-details)
  - [Migrad Algorithm](#migrad-algorithm)
  - [Simplex Algorithm](#simplex-algorithm)
  - [Parameter Transforms](#parameter-transforms)
  - [Numerical Gradient](#numerical-gradient)
  - [Line Search](#line-search)
  - [DFP Inverse Hessian Update](#dfp-inverse-hessian-update)
  - [Positive-Definite Correction](#positive-definite-correction)
- [Architecture](#architecture)
  - [Module Map](#module-map)
  - [Internal vs External Parameters](#internal-vs-external-parameters)
  - [C++ to Rust Mapping](#c-to-rust-mapping)
- [Constants Reference](#constants-reference)
- [Troubleshooting](#troubleshooting)

---

## Quick Start

Add to `Cargo.toml`:

```toml
[dependencies]
minuit2 = "0.2"
```

Minimize the Rosenbrock function:

```rust
use minuit2::MnMigrad;

let result = MnMigrad::new()
    .add("x", 0.0, 0.1)   // name, start value, initial error
    .add("y", 0.0, 0.1)
    .minimize(&|p: &[f64]| {
        (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2)
    });

println!("Valid: {}", result.is_valid());
println!("Minimum at: {:?}", result.params());
println!("Function value: {}", result.fval());
println!("Iterations (nfcn): {}", result.nfcn());
println!("{result}"); // full formatted output
```

---

## Minimizers

### MnMigrad

**Quasi-Newton minimizer using the Davidon-Fletcher-Powell (DFP) update.** This is the recommended minimizer for smooth functions. It converges quadratically near the minimum and produces an approximate covariance matrix.

```rust
use minuit2::MnMigrad;

let result = MnMigrad::new()
    .add("amplitude", 8.0, 1.0)
    .add("mean", 4.0, 0.5)
    .add_lower_limited("sigma", 1.5, 0.5, 0.01)
    .with_strategy(1)     // 0=fast, 1=default, 2=careful
    .tolerance(0.1)       // tighter convergence (default: 1.0)
    .max_fcn(5000)        // call limit (default: 200 + 100n + 5n²)
    .minimize(&my_chi2);
```

**When to use:** Most cases. Especially good for chi-square fits, likelihood fits, and any smooth objective function.

**When NOT to use:** Non-differentiable functions, functions with discontinuities, or very noisy objectives. Use `MnSimplex` instead.

### MnSimplex

**Nelder-Mead simplex minimizer (Minuit variant).** Derivative-free. Robust but slow — uses only function evaluations, no gradient information.

```rust
use minuit2::MnSimplex;

let result = MnSimplex::new()
    .add("x", 0.0, 0.1)
    .add("y", 0.0, 0.1)
    .tolerance(0.01)
    .minimize(&|p: &[f64]| {
        (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2)
    });
```

**When to use:** Noisy or non-smooth functions, or as a pre-conditioning step before Migrad.

**Note:** Minuit's simplex is NOT textbook Nelder-Mead. It uses rho-extrapolation from the original Fortran MINUIT and has no shrink step.

---

## Defining a Function (FCN)

### Using a closure (simplest)

Any `Fn(&[f64]) -> f64` automatically implements `FCN` with `error_def() = 1.0`:

```rust
let result = MnMigrad::new()
    .add("x", 0.0, 0.1)
    .minimize(&|p: &[f64]| p[0] * p[0]);
```

### Using a struct (custom error_def)

Implement the `FCN` trait for log-likelihood fits or custom error definitions:

```rust
use minuit2::FCN;

struct MyLikelihood {
    data: Vec<f64>,
}

impl FCN for MyLikelihood {
    fn value(&self, par: &[f64]) -> f64 {
        // -2 * log(likelihood)
        self.data.iter()
            .map(|&x| {
                let mu = par[0];
                let sigma = par[1];
                let z = (x - mu) / sigma;
                z * z + 2.0 * sigma.ln()
            })
            .sum()
    }

    fn error_def(&self) -> f64 {
        1.0  // 1.0 for chi-square, 0.5 for -log(L)
    }
}
```

### The `error_def()` convention

| Fit type | `error_def()` | Meaning |
|----------|---------------|---------|
| Chi-square (`Σ(data-model)²/σ²`) | `1.0` | Δχ² = 1 for 1σ errors |
| Negative log-likelihood (`-ln L`) | `0.5` | Δ(-ln L) = 0.5 for 1σ |
| `-2 ln L` | `1.0` | Same as chi-square |

This affects how Minuit interprets parameter errors and the EDM convergence criterion.

---

## Parameters

Parameters are added via the builder pattern. Each parameter has:
- **name**: string identifier for lookup
- **value**: starting value
- **error**: initial step size (NOT the final error — this seeds the gradient calculation)

### Free Parameters

```rust
.add("mass", 125.0, 0.5)  // name, start, step
```

### Bounded Parameters

```rust
// Both bounds
.add_limited("sigma", 2.0, 0.5, 0.01, 100.0)  // name, start, step, lower, upper

// Lower bound only
.add_lower_limited("energy", 100.0, 10.0, 0.0)  // name, start, step, lower

// Upper bound only
.add_upper_limited("fraction", 0.5, 0.1, 1.0)  // name, start, step, upper
```

**How bounds work internally:** Bounded parameters are transformed to an unbounded internal space via smooth transformations:
- Both bounds → `sin` transform: `ext = lower + (upper-lower) * (sin(int)+1)/2`
- Lower only → `sqrt_low` transform: `ext = lower - 1 + sqrt(int² + 1)`
- Upper only → `sqrt_up` transform: `ext = upper + 1 - sqrt(int² + 1)`

The optimizer works entirely in the internal space. Results are transformed back to external (user) space.

### Fixed Parameters

```rust
.add("x", 0.0, 0.5)
.add("y", 0.0, 0.5)
.fix(1)  // fix parameter at index 1 (y) at its start value
```

Fixed parameters are excluded from the internal optimization space but still appear in the FCN parameter array at their fixed values.

### Constant Parameters

```rust
.add_const("pi", std::f64::consts::PI)
```

Constants cannot be released. They always pass their value to the FCN.

### Parameter ordering

Parameters are indexed by the order they are added. The FCN receives a slice `p: &[f64]` where `p[0]` is the first added parameter, `p[1]` the second, etc. — including fixed/constant parameters at their fixed values.

---

## Interpreting Results

### FunctionMinimum

The `minimize()` call returns a `FunctionMinimum`:

```rust
let result = minimizer.minimize(&fcn);

// Key accessors
result.is_valid()          // true if converged properly
result.fval()              // function value at minimum
result.edm()               // estimated distance to minimum
result.nfcn()              // total function calls used
result.params()            // Vec<f64> of parameter values (external space)
result.up()                // error definition used
result.reached_call_limit() // hit maxfcn
result.is_above_max_edm()  // EDM > threshold but stopped

// Formatted output
println!("{result}");
```

**Validity:** A result is valid if:
1. The minimization converged (EDM below threshold)
2. The call limit was not reached
3. The parameters are finite

### User Parameter State

For named access to individual parameters:

```rust
let state = result.user_state();

// By name
let mass = state.value("mass").unwrap();
let mass_err = state.error("mass").unwrap();

// By index
let p = state.parameter(0);
println!("{}: {} +/- {}", p.name(), p.value(), p.error());

// Metadata
state.fval()                  // function value
state.edm()                   // estimated distance to minimum
state.nfcn()                  // total function calls
state.variable_parameters()   // number of free parameters
```

---

## Convergence Control

### Tolerance

The `tolerance` parameter controls how precisely the minimizer converges:

```rust
.tolerance(1.0)   // default — good for most cases
.tolerance(0.1)   // 10x tighter — for precision fits
.tolerance(10.0)  // 10x looser — for fast rough minimization
```

**How it works:**
- Migrad: `edmval = tolerance * error_def * 0.002` (the 0.002 is F77 Minuit compatibility)
- Simplex: `minedm = tolerance * error_def * 0.001`

The minimizer stops when EDM (estimated distance to minimum) drops below `edmval`.

### Maximum Function Calls

```rust
.max_fcn(10000)  // override default
```

Default: `200 + 100n + 5n²` where `n` = number of variable parameters.

If the call limit is reached before convergence, the result will have `reached_call_limit() == true` and `is_valid() == false`.

### Strategy

Controls the effort spent on gradient calculation and Hessian refinement:

```rust
.with_strategy(0)  // low: fewer gradient cycles, larger tolerances
.with_strategy(1)  // medium (default): balanced
.with_strategy(2)  // high: more gradient cycles, tighter tolerances
```

| Property | Low (0) | Medium (1) | High (2) |
|----------|---------|------------|----------|
| Gradient cycles | 2 | 3 | 5 |
| Gradient step tolerance | 0.5 | 0.3 | 0.1 |
| Gradient tolerance | 0.1 | 0.05 | 0.02 |

Higher strategy = more function calls per iteration but more robust convergence.

---

## Algorithm Details

### Migrad Algorithm

Migrad implements a quasi-Newton method with the DFP inverse Hessian update. The algorithm:

1. **Seed:** Evaluate FCN at starting point, compute numerical gradient via 2-point central differences, build initial V₀ = diag(1/g2_i)
2. **Iterate:**
   - Compute Newton step: `step = -V * grad`
   - Verify descent direction: `gdel = step · grad < 0` (force V pos.def. if not)
   - Line search: parabolic interpolation along step → (λ, f_new)
   - Update parameters: `p_new = p_old + λ * step`
   - Compute new gradient (2-point central differences)
   - DFP update of V: rank-2 correction from parameter and gradient changes
   - EDM = `0.5 * g^T * V * g * (1 + 3*dcovar)`
   - Stop when EDM < tolerance * error_def * 0.002
3. **Second pass:** If first pass fails, retry with 1.3x budget

### Simplex Algorithm

Minuit's variant of Nelder-Mead simplex:

1. Build N+1 vertex simplex from starting point
2. Iterate: reflect worst vertex through centroid
   - If reflected is better than best: try expansion, then rho-extrapolation
   - If reflected is worse than worst: try contraction (no shrink step)
3. Convergence: both current AND previous EDM below threshold
4. Post-loop: try centroid as potential final point

### Parameter Transforms

Bounded parameters use smooth bijective transforms between external [lo, hi] and internal (-∞, +∞):

| Bound type | Transform | Formula (int→ext) |
|------------|-----------|-------------------|
| Both bounds | Sin | `lo + (hi-lo)*(sin(int)+1)/2` |
| Lower only | SqrtLow | `lo - 1 + sqrt(int²+1)` |
| Upper only | SqrtUp | `hi + 1 - sqrt(int²+1)` |

The Jacobian `d(ext)/d(int)` is used to transform gradients and errors between spaces.

### Numerical Gradient

Two-point central differences with adaptive step sizing:

```
g_i = (f(x+h) - f(x-h)) / 2h
g2_i = (f(x+h) + f(x-h) - 2*f(x)) / h²
```

Step size `h` is refined over `ncycles` iterations:
1. Optimal step: `h_opt = sqrt(dfmin / |g2|)` (balances truncation vs roundoff)
2. Bounded params: cap h at 0.5
3. Clamp: `h ∈ [8*eps²*|x|, 10*h_prev]`
4. Converge when both step and gradient stabilize within strategy-dependent tolerances

### Line Search

Parabolic interpolation along the Newton step direction:

1. Evaluate at λ=0 (current point) and λ=1 (full step)
2. Fit quadratic through 2 points + directional derivative → interpolated λ
3. Iterate (up to 12 times): fit parabola through 3 best points, evaluate at minimum
4. Return best (λ, f) found

### DFP Inverse Hessian Update

After each step, the inverse Hessian V is updated using the Davidon-Fletcher-Powell formula with an optional BFGS rank-1 correction:

```
dx = p_new - p_old       (parameter change)
dg = g_new - g_old       (gradient change)
delgam = dx · dg
vg = V * dg
gvg = dg · vg

V_new = V + outer(dx)/delgam - outer(vg)/gvg           (rank-2)
if delgam > gvg:
    V_new += gvg * outer(dx/delgam - vg/gvg)           (rank-1 BFGS correction)
```

The `dcovar` metric tracks update magnitude: `dcovar = 0.5 * (old_dcovar + |ΔV|/|V_new|)`. It starts at 1.0 and approaches 0 as V converges to the true inverse Hessian.

### Positive-Definite Correction

If V is not positive-definite (detected when `step · grad > 0`), the `make_pos_def` correction is applied:

1. Shift negative diagonals: `diag += 0.5 + eps - min(diag)`
2. Normalize to correlation matrix
3. Eigendecompose: find min/max eigenvalues
4. If min eigenvalue < eps * max eigenvalue: shift all eigenvalues by `0.001*max - min`
5. Reconstruct and un-normalize

---

## Architecture

### Module Map

```
src/
├── lib.rs                 # Module declarations, re-exports
├── fcn.rs                 # FCN / FCNGradient traits
├── mn_fcn.rs              # MnFcn: call-counting wrapper (int→ext transform)
├── parameter.rs           # MinuitParameter (single parameter)
├── precision.rs           # MnMachinePrecision (eps, eps2)
├── strategy.rs            # MnStrategy (low/medium/high presets)
├── application.rs         # default_max_fcn(), DEFAULT_TOLERANCE
├── print.rs               # Display impl for FunctionMinimum
├── user_parameters.rs     # MnUserParameters (parameter collection)
├── user_parameter_state.rs # MnUserParameterState (fitted state + covariance)
├── user_covariance.rs     # MnUserCovariance (upper-triangle storage)
├── user_transformation.rs # MnUserTransformation (ext↔int index/value mapping)
│
├── parabola.rs            # MnParabola + MnParabolaPoint (quadratic fit)
├── linesearch.rs          # mn_linesearch (parabolic line search)
├── posdef.rs              # make_pos_def (eigenvalue shift)
│
├── minimum/               # Result data structures
│   ├── mod.rs             #   FunctionMinimum (top-level result)
│   ├── state.rs           #   MinimumState (params + error + gradient)
│   ├── parameters.rs      #   MinimumParameters (internal param vector)
│   ├── error.rs           #   MinimumError (inverse Hessian)
│   ├── gradient.rs        #   FunctionGradient (grad + g2 + gstep)
│   └── seed.rs            #   MinimumSeed (initial state + trafo)
│
├── gradient/              # Gradient calculators
│   ├── mod.rs             #   GradientCalculator trait
│   ├── initial.rs         #   InitialGradientCalculator (heuristic from step sizes)
│   └── numerical.rs       #   Numerical2PGradientCalculator (2-point central diff)
│
├── simplex/               # Nelder-Mead simplex minimizer
│   ├── mod.rs             #   MnSimplex (public builder API)
│   ├── builder.rs         #   SimplexBuilder (iteration loop)
│   ├── minimizer.rs       #   SimplexMinimizer (composes seed + builder)
│   ├── parameters.rs      #   SimplexParameters (vertex storage)
│   └── seed.rs            #   SimplexSeedGenerator
│
├── migrad/                # Variable-metric (Migrad) minimizer
│   ├── mod.rs             #   MnMigrad (public builder API)
│   ├── builder.rs         #   VariableMetricBuilder (iteration loop + DFP)
│   ├── minimizer.rs       #   VariableMetricMinimizer (composes seed + builder)
│   └── seed.rs            #   MigradSeedGenerator
│
└── transform/             # Parameter space transformations
    ├── mod.rs             #   ParameterTransform trait
    ├── sin.rs             #   SinTransform (doubly-bounded)
    ├── sqrt_low.rs        #   SqrtLowTransform (lower-bounded)
    └── sqrt_up.rs         #   SqrtUpTransform (upper-bounded)
```

### Internal vs External Parameters

Minuit2 operates in two parameter spaces:

| Space | Description | Used by |
|-------|-------------|---------|
| **External** | User-facing values with bounds and fixed params | FCN, user API |
| **Internal** | Unbounded, only variable params | Optimizer core |

The `MnUserTransformation` manages the mapping:
- Fixed parameters are excluded from the internal space
- Bounded parameters are transformed via sin/sqrt transforms
- `ext_of_int(i)` maps internal index → external index
- `int_of_ext(i)` maps external index → internal index (None if fixed)

The `MnFcn` wrapper accepts internal parameters, transforms them to external, and calls the user's FCN.

### C++ to Rust Mapping

| C++ (GooFit/Minuit2) | Rust (minuit2-rs) | Notes |
|-----------------------|-------------------|-------|
| `BasicMinimumState` + `MinimumState` | `MinimumState` | Flat struct, no smart pointers |
| `BasicMinimumError` + `MinimumError` | `MinimumError` | Direct ownership |
| `BasicFunctionGradient` + `FunctionGradient` | `FunctionGradient` | Direct ownership |
| `BasicMinimumSeed` + `MinimumSeed` | `MinimumSeed` | Direct ownership |
| `BasicFunctionMinimum` + `FunctionMinimum` | `FunctionMinimum` | Direct ownership |
| `MnRefCountedPointer<T>` | Rust ownership / `Clone` | No reference counting needed |
| `LASymMatrix` / `LAVector` | `DMatrix<f64>` / `DVector<f64>` | nalgebra replaces 28 LA files |
| `DavidonErrorUpdator` | Inline in `builder.rs` | No separate trait needed |
| `VariableMetricEDMEstimator` | Inline `0.5 * g^T * V * g` | One-liner in Rust |
| `MnApplication` abstract class | Builder pattern on `MnMigrad`/`MnSimplex` | No inheritance hierarchy |
| `operator()` on calculators | Named methods (`compute`, `compute_with_previous`) | Avoids Fn trait name collision |

---

## Constants Reference

| Constant | Value | Location | Purpose |
|----------|-------|----------|---------|
| EDM scale factor | 0.002 | Migrad builder | F77 Minuit compatibility |
| Dcovar EDM weight | 3.0 | Migrad builder | Inflates EDM for uncertain V |
| Second pass budget | 1.3x | Migrad builder | Retry with more FCN calls |
| Final EDM tolerance | 10x edmval | Migrad minimizer | Warn if EDM still large |
| Line search maxiter | 12 | linesearch.rs | Parabolic interpolation limit |
| Line search slambg | 5.0 | linesearch.rs | Max initial step multiplier |
| Line search alpha | 2.0 | linesearch.rs | Step expansion factor |
| Line search toler | 0.05 | linesearch.rs | Convergence tolerance |
| Gradient gsmin | 8*eps2*(|x|+eps2) | numerical.rs | Minimum step size |
| Gradient max step | 10*gstep | numerical.rs | Maximum step size |
| Bounded param cap | 0.5 | numerical.rs | Step limit for transforms |
| Simplex rhomin | 4.0 | simplex/builder.rs | Rho extrapolation minimum |
| Simplex rhomax | 8.0 | simplex/builder.rs | Rho extrapolation maximum |
| Default max FCN | 200+100n+5n² | application.rs | Call limit formula |

---

## Troubleshooting

### "Minimization did not converge" (is_valid = false)

1. **reached_call_limit = true**: Increase `.max_fcn()` or start closer to the minimum
2. **is_above_max_edm = true**: Lower `.tolerance()` or check that the function has a well-defined minimum
3. **Check starting values**: Bad initial parameters can lead to poor convergence. The initial `error` (step size) should roughly match the expected distance to the minimum.

### Slow convergence

- Use `MnMigrad` instead of `MnSimplex` for smooth functions
- Use `.with_strategy(0)` for faster but less precise convergence
- Reduce the number of free parameters by fixing those you know

### Bounded parameters converge slowly

This is expected — the sin/sqrt transforms distort the parameter space near boundaries. Try:
- Starting further from the boundary
- Using wider bounds if possible
- Increasing `.max_fcn()`

### Parameters at boundary

If a parameter sits exactly at its bound, it may indicate the minimum is outside the allowed range. Consider relaxing the bound or rethinking the model.

### EDM meaning

EDM (Estimated Distance to Minimum) measures how far the current point is from the true minimum, estimated as `0.5 * g^T * V * g`. For a well-converged fit:
- EDM should be much smaller than `tolerance * error_def * 0.002`
- Typical converged values: 1e-6 to 1e-10

---

## Test Suite

58 tests total:
- **42 unit tests**: parameter handling, transforms, precision, gradient, parabola, line search, positive-definite correction, strategy
- **8 Migrad integration tests**: Rosenbrock 2D, quadratic bowl, 5D quadratic, bounded params, fixed params, Gaussian fit, Migrad vs Simplex comparison, display output
- **6 Simplex integration tests**: Rosenbrock, quadratic, Gaussian, bounded, fixed, display
- **2 doctests**: lib.rs quick start examples

Run all tests:
```bash
cargo test
```

Run only Migrad tests:
```bash
cargo test --test migrad
```
