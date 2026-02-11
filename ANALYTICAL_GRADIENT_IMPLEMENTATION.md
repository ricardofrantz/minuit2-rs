# Analytical Gradient Support in Migrad - Implementation Summary

## Overview

This implementation adds support for user-provided analytical gradients to the Migrad minimizer in minuit2-rs. Users can now provide explicit gradient calculations instead of relying solely on numerical differentiation, potentially reducing function evaluation counts significantly.

## Architecture

### 1. New File: `src/gradient/analytical.rs`

**AnalyticalGradientCalculator** - Transforms user-provided gradients from external parameter space to internal space:

```rust
pub fn compute(
    fcn: &dyn FCNGradient,
    trafo: &MnUserTransformation,
    params: &MinimumParameters,
) -> FunctionGradient
```

**Key Design Decisions:**

- User provides gradients in external parameter space (simplest interface)
- Transform to internal space using chain rule: `g_int[i] = g_ext[i] * dext/dint[i]`
- Get derivative via `trafo.dint2ext(ext_idx, internal_val) -> f64`
- Compute g2 (second derivative heuristic) using same logic as `InitialGradientCalculator`:
  - `g2[i] = 2 * error_def / dirin^2` where dirin from parameter error
- Compute gstep (line search step sizes) as `0.1 * dirin` (minimum scaled by machine precision)
- Handles bounded parameters correctly via transform derivatives

### 2. Modified Files

#### `src/gradient/mod.rs`
- Export new `AnalyticalGradientCalculator`

#### `src/minimum/gradient.rs`
- Added `set_analytical(&mut self, analytical: bool)` method to mark user gradients

#### `src/migrad/seed.rs`
- New method: `MigradSeedGenerator::generate_with_gradient()`
- Creates initial seed using analytical gradients (only one FCN eval for function value)
- Builds V₀ = diag(1/g2_i) same as numerical path

#### `src/migrad/builder.rs`
- New method: `VariableMetricBuilder::minimum_with_gradient()`
- New private method: `iterate_with_gradient()` - main iteration loop using analytical gradients
- Key difference: calls `AnalyticalGradientCalculator::compute()` each iteration instead of numerical
- DFP update and line search unchanged - both work with analytical gradients

#### `src/migrad/minimizer.rs`
- New method: `VariableMetricMinimizer::minimize_with_gradient()`
- Routes to analytical seed generator and builder

#### `src/migrad/mod.rs`
- New public method: `MnMigrad::minimize_grad(&self, fcn: &dyn FCNGradient) -> FunctionMinimum`
- User-facing API entry point for analytical gradients

## Usage Example

```rust
use minuit2::{FCN, FCNGradient, MnMigrad};

struct MyFunction;

impl FCN for MyFunction {
    fn value(&self, p: &[f64]) -> f64 {
        // User's objective function
        p[0] * p[0] + 4.0 * p[1] * p[1]
    }
}

impl FCNGradient for MyFunction {
    fn gradient(&self, p: &[f64]) -> Vec<f64> {
        // User's analytical gradient in external parameter space
        vec![2.0 * p[0], 8.0 * p[1]]
    }
}

let result = MnMigrad::new()
    .add("x", 3.0, 0.1)
    .add("y", 2.0, 0.1)
    .minimize_grad(&MyFunction);  // Use analytical gradients
```

## Implementation Details

### Gradient Transformation

For each internal parameter i:
1. Get external index: `ext_idx = trafo.ext_of_int(i)`
2. Get transform derivative: `dext_dint = trafo.dint2ext(ext_idx, internal_val)`
3. User provides gradient in external space: `g_ext = fcn.gradient(external_params)[ext_idx]`
4. Apply chain rule: `g_int[i] = g_ext[i] * dext_dint`

### Bounded Parameters

The transform derivatives automatically account for parameter bounds:
- Sin transform for two-sided bounds: `dext/dint = (upper-lower)/2 * cos(internal)`
- Sqrt transforms for one-sided bounds: scaled appropriately
- The computation is automatic via `dint2ext()` method

### g2 and gstep Computation

Even though user only provides first derivatives, g2 (second derivative estimate) and gstep (step sizes) are computed using the same heuristics as `InitialGradientCalculator`:

```rust
let werr = trafo.parameters()[ext_idx].error();  // Parameter error from user
// ... compute dirin from forward/backward external steps ...
let g2i = 2.0 * error_def / (dirin * dirin);
let gstepi = gsmin.max(0.1 * dirin);
```

This ensures line search and iteration work properly without requiring user to provide Hessian.

## Integration Points

**Unchanged:**
- `MnFcn::call()` - still counts function evaluations (for FCN value only, not gradient)
- Line search (`mn_linesearch`) - works identically with analytical gradients
- DFP update - works identically (only needs gradient vectors)
- Positive-definiteness checking and fallback to steepest descent
- Convergence detection (EDM tolerance)
- Re-seeding on first pass failure

**Changed:**
- Seed generation: analytical path skips numerical gradient computation
- Iteration loop: analytical path calls AnalyticalGradientCalculator instead of Numerical2PGradientCalculator

## Benefits

1. **Reduced Function Calls**: No numerical differentiation needed
   - Numerical: 2n+1 FCN calls per gradient (central differences)
   - Analytical: 1 FCN call per iteration for line search only

2. **Higher Accuracy**: User-provided gradients typically more accurate than numerical

3. **Seamless Integration**: FCNGradient trait extends FCN, so analytical code path is optional

## Testing

**Unit Tests** (src/gradient/analytical.rs):
- `analytical_gradient_quadratic` - correctness on unbounded quadratic
- `analytical_gradient_bounded_param` - correctness with bounded parameters

**Integration Tests** (tests/migrad_analytical.rs):
- `migrad_analytical_quadratic` - full minimization with analytical gradients
- `migrad_analytical_vs_numerical_quadratic` - both paths reach same minimum
- `migrad_analytical_rosenbrock` - harder function convergence
- `migrad_analytical_with_bounds` - bounded parameter support

**All Tests Pass**: 82 total tests (50 unit + 28 integration + 2 doctests)

## Limitations and Future Work

1. **Hessian Still Numerical**: If user wants analytical Hessian, would need separate MnHesse support
2. **No Auto-Differentiation**: User must provide gradient manually (no AD integration)
3. **Strategy Parameter Unused in Analytical Path**: `strategy` passed but not used in gradient calc
4. **Gradient Validation**: No automatic check that user gradient is correct (numerical validation could be added)

## Files Modified

```
src/gradient/analytical.rs          [NEW] AnalyticalGradientCalculator
src/gradient/mod.rs                 [MOD] Export analytical module
src/migrad/mod.rs                   [MOD] Add minimize_grad() method
src/migrad/minimizer.rs             [MOD] Add minimize_with_gradient()
src/migrad/seed.rs                  [MOD] Add generate_with_gradient()
src/migrad/builder.rs               [MOD] Add minimum_with_gradient() and iterate_with_gradient()
src/minimum/gradient.rs             [MOD] Add set_analytical() method
tests/migrad_analytical.rs          [NEW] Integration tests
```

**Lines Changed:**
- New: ~275 lines (analytical.rs)
- New: ~180 lines (tests/migrad_analytical.rs)
- Modified: ~50 lines (various files)
- Total: ~505 lines added

## Backward Compatibility

✓ Fully backward compatible:
- Existing `minimize(&dyn FCN)` method unchanged
- All existing tests pass without modification
- FCNGradient is optional trait extension
