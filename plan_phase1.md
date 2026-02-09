# Phase 1: Core Types & Infrastructure (2-4 weeks)

No runnable minimizer yet — this builds the type system everything depends on.

## Translation Order

Work bottom-up: types with no dependencies first, then composites.

### 1. Machine precision & config
Tiny files. Do first — everything else references them.

| C++ | → Rust | Notes |
|-----|--------|-------|
| [MnConfig.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnConfig.h) | `config.rs` | Thread-local settings, `MINOS_RELEASE` |
| [MnTiny.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnTiny.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnTiny.cxx) | `tiny.rs` | Volatile trick to find machine epsilon |
| [MnMachinePrecision.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnMachinePrecision.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnMachinePrecision.cxx) | `precision.rs` | `eps()`, `eps2()` — use `f64::EPSILON` directly |

### 2. Linear algebra decision
The C++ uses custom `LASymMatrix`/`LAVector` with expression templates (`ABObj`, `ABProd`, `ABSum`) and hand-rolled BLAS (`mndasum`, `mnddot`, etc.). **Replace all of this with `nalgebra`.**

Create a thin wrapper module:
```rust
// src/linalg.rs
pub type MnVec = nalgebra::DVector<f64>;
pub type MnMat = nalgebra::DMatrix<f64>;
```

Read [LASymMatrix.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/LASymMatrix.h) and [LAVector.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/LAVector.h) to understand the API surface (indexing, size, data access) — then map those operations to `nalgebra` equivalents.

### 3. Parameter transforms
Short, self-contained math. Port verbatim.

| C++ | → Rust |
|-----|--------|
| [SinParameterTransformation.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SinParameterTransformation.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SinParameterTransformation.cxx) | `transform/sin.rs` — both upper+lower bounded |
| [SqrtLowParameterTransformation.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SqrtLowParameterTransformation.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SqrtLowParameterTransformation.cxx) | `transform/sqrt_low.rs` — lower bounded only |
| [SqrtUpParameterTransformation.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SqrtUpParameterTransformation.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SqrtUpParameterTransformation.cxx) | `transform/sqrt_up.rs` — upper bounded only |

Each has 3 methods: `int2ext(value)`, `ext2int(value)`, `dint2ext(value)` (derivative).

### 4. Function interface trait
| C++ | → Rust |
|-----|--------|
| [GenericFunction.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/GenericFunction.h) | Skip — Rust traits replace this |
| [FCNBase.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FCNBase.h) | `trait FCN { fn call(&self, par: &[f64]) -> f64; fn error_def(&self) -> f64; }` |
| [FCNGradientBase.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FCNGradientBase.h) | `trait FCNGradient: FCN { fn gradient(&self, par: &[f64]) -> Vec<f64>; }` |

Also support closures: `impl FCN for F where F: Fn(&[f64]) -> f64`.

### 5. Parameter management
Most complex part of Phase 1. These files are tightly coupled — read all headers before starting.

| C++ | → Rust | LOC |
|-----|--------|-----|
| [MinuitParameter.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinuitParameter.h) | `parameter.rs` — single param (name, value, error, limits, fixed) | ~100 |
| [MnUserParameters.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnUserParameters.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnUserParameters.cxx) | `user_parameters.rs` — public API | ~200 |
| [MnUserParameterState.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnUserParameterState.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnUserParameterState.cxx) | `user_parameter_state.rs` — full state (params + covariance + global CC) | ~500 |
| [MnUserCovariance.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnUserCovariance.h) | `user_covariance.rs` — upper-triangle covariance matrix | ~50 |
| [MnUserTransformation.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnUserTransformation.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnUserTransformation.cxx) | `user_transformation.rs` — internal↔external param mapping | ~300 |
| [MnUserFcn.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnUserFcn.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnUserFcn.cxx) | `user_fcn.rs` — wraps FCN with transforms | ~50 |
| [MnVectorTransform.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnVectorTransform.h) | `vector_transform.rs` — batch ext↔int | ~30 |

### 6. Strategy
| C++ | → Rust |
|-----|--------|
| [MnStrategy.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnStrategy.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnStrategy.cxx) | `strategy.rs` — 3 presets controlling gradient steps, Hesse behavior |

### 7. Result types
These wrap the internal state returned by minimizers.

| C++ | → Rust |
|-----|--------|
| [BasicMinimumParameters.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicMinimumParameters.h) | `minimum/parameters.rs` |
| [MinimumParameters.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumParameters.h) | (same — smart pointer wrapper in C++) |
| [BasicMinimumError.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicMinimumError.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/BasicMinimumError.cxx) | `minimum/error.rs` — inverse Hessian + status flags |
| [MinimumError.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumError.h) | (same) |
| [BasicMinimumSeed.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicMinimumSeed.h) | `minimum/seed.rs` |
| [MinimumSeed.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumSeed.h) | (same) |
| [BasicMinimumState.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicMinimumState.h) | `minimum/state.rs` |
| [MinimumState.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumState.h) | (same) |
| [BasicFunctionGradient.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicFunctionGradient.h) | `minimum/gradient.rs` |
| [FunctionGradient.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FunctionGradient.h) | (same) |
| [BasicFunctionMinimum.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicFunctionMinimum.h) | `minimum/mod.rs` — the top-level result |
| [FunctionMinimum.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FunctionMinimum.h) | (same) |

**Note**: The C++ uses `Basic*` + smart-pointer wrapper pairs. In Rust, flatten to single structs — ownership replaces reference counting.

### 8. Minimizer framework (abstract base)
| C++ | → Rust |
|-----|--------|
| [GradientCalculator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/GradientCalculator.h) | `trait GradientCalculator` |
| [MinimumErrorUpdator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumErrorUpdator.h) | `trait MinimumErrorUpdator` |
| [MinimumSeedGenerator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumSeedGenerator.h) | `trait MinimumSeedGenerator` |
| [MinimumBuilder.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumBuilder.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MinimumBuilder.cxx) | `trait MinimumBuilder` |
| [FunctionMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FunctionMinimizer.h) | `trait FunctionMinimizer` |
| [ModularFunctionMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ModularFunctionMinimizer.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/ModularFunctionMinimizer.cxx) | Base impl composing seed+builder+updator |
| [MnFcn.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnFcn.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnFcn.cxx) | `mn_fcn.rs` — function call counter wrapper |
| [MnApplication.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnApplication.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnApplication.cxx) | `application.rs` — public `minimize()` entry point |

### 9. Printing
| C++ | → Rust |
|-----|--------|
| [MnPrint.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnPrint.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnPrint.cxx) + [Impl.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnPrintImpl.cxx) | `print.rs` — use `Display` trait + `log` crate |

## Deliverables
- [ ] `Cargo.toml` with `nalgebra` dependency
- [ ] All types compile, unit tests pass for parameter add/fix/release/limit
- [ ] Parameter transforms tested against C++ reference values
- [ ] `FCN` trait works with closures and structs
- [ ] `FunctionMinimum` can be constructed and queried
