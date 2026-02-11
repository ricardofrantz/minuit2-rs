# Analytical Gradient Implementation - Key Changes Reference

## File-by-File Changes

### 1. NEW: `/src/gradient/analytical.rs` (275 lines)

Core calculator that transforms user gradients:

```rust
pub struct AnalyticalGradientCalculator;

impl AnalyticalGradientCalculator {
    /// Compute gradient from user-provided analytical gradient.
    pub fn compute(
        fcn: &dyn FCNGradient,
        trafo: &MnUserTransformation,
        params: &MinimumParameters,
    ) -> FunctionGradient {
        // 1. Get external params
        let external_vals = trafo.transform(internal_vec.as_slice());
        
        // 2. Call user gradient (in external space)
        let ext_gradient = fcn.gradient(&external_vals);
        
        // 3. For each internal param i:
        //    - Get transform derivative: dext_dint = trafo.dint2ext(ext_idx, int_val)
        //    - Apply chain rule: g_int[i] = g_ext[i] * dext_dint
        //    - Compute g2 heuristic: g2[i] = 2*error_def / dirin^2
        //    - Compute gstep: gstepi = 0.1 * dirin
        
        // 4. Return FunctionGradient marked as analytical
    }
}
```

**Key Points:**
- Transform derivatives from `MnUserTransformation::dint2ext(ext_idx, internal_val)`
- Chain rule handles bounded parameters automatically
- g2 and gstep computed using same heuristics as InitialGradientCalculator
- Result marked with `.set_analytical(true)`

### 2. MODIFIED: `/src/migrad/seed.rs`

Add seed generation for analytical path:

```rust
impl MigradSeedGenerator {
    // Existing numerical path (unchanged)
    pub fn generate(fcn: &MnFcn, trafo: &MnUserTransformation, strategy: &MnStrategy) -> MinimumSeed

    // NEW: Analytical path
    pub fn generate_with_gradient(
        fcn: &dyn FCNGradient,
        trafo: &MnUserTransformation,
        _strategy: &MnStrategy,
    ) -> MinimumSeed {
        // 1. Evaluate FCN at starting point (for function value only)
        let fval = fcn.value(&trafo.transform(&int_values));
        
        // 2. Compute analytical gradient using AnalyticalGradientCalculator
        let gradient = AnalyticalGradientCalculator::compute(fcn, trafo, &params);
        
        // 3. Build V₀ = diag(1/g2_i) same as numerical
        // 4. Compute initial EDM = 0.5 * g^T * V * g
        // 5. Return MinimumSeed
    }
}
```

**Difference from numerical:**
- No gradient evaluation loop (one FCN call only)
- Direct call to AnalyticalGradientCalculator
- nfcn counter starts at 1 (just the initial FCN eval)

### 3. MODIFIED: `/src/migrad/builder.rs`

Add iteration loop for analytical path:

```rust
impl VariableMetricBuilder {
    // Existing numerical path
    pub fn minimum(fcn: &MnFcn, seed: &MinimumSeed, ...) -> Vec<MinimumState>
    
    // NEW: Analytical path (top-level)
    pub fn minimum_with_gradient(
        fcn: &MnFcn,
        gradient_fcn: &dyn FCNGradient,
        seed: &MinimumSeed,
        _strategy: &MnStrategy,
        maxfcn: usize,
        edmval: f64,
    ) -> Vec<MinimumState> {
        // Calls iterate_with_gradient()
    }
    
    // NEW: Core iteration loop for analytical gradients
    fn iterate_with_gradient(
        fcn: &MnFcn,
        gradient_fcn: &dyn FCNGradient,
        seed: &MinimumSeed,
        maxfcn: usize,
        edmval: f64,
    ) -> Vec<MinimumState> {
        loop {
            // 1. Newton step: step = -V * grad
            // 2. Check positive-definiteness
            // 3. Line search (unchanged)
            // 4. Update parameters
            
            // 5. DIFFERENT: Compute new gradient using analytical
            let new_gradient = AnalyticalGradientCalculator::compute(
                gradient_fcn,
                seed.trafo(),
                &new_params,
            );
            
            // 6. DFP update (unchanged)
            // 7. Check convergence (unchanged)
        }
    }
}
```

**Key Difference:**
- Line 5 replaces `Numerical2PGradientCalculator::compute_with_previous()`
- Everything else (Newton step, line search, DFP) identical

### 4. MODIFIED: `/src/migrad/minimizer.rs`

Route between numerical and analytical paths:

```rust
impl VariableMetricMinimizer {
    // Existing path (unchanged)
    pub fn minimize(fcn: &MnFcn, trafo: &MnUserTransformation, ...) -> FunctionMinimum {
        let seed = MigradSeedGenerator::generate(fcn, trafo, strategy);
        let states = VariableMetricBuilder::minimum(fcn, &seed, strategy, maxfcn, edmval);
        // ... check outcome ...
    }
    
    // NEW: Analytical path
    pub fn minimize_with_gradient(
        fcn: &dyn FCNGradient,
        trafo: &MnUserTransformation,
        strategy: &MnStrategy,
        maxfcn: usize,
        tolerance: f64,
    ) -> FunctionMinimum {
        // Route to analytical seed generator
        let seed = MigradSeedGenerator::generate_with_gradient(fcn, trafo, strategy);
        
        // Create MnFcn for call counting
        let mn_fcn = MnFcn::new(fcn, trafo);
        
        // Route to analytical builder
        let states = VariableMetricBuilder::minimum_with_gradient(
            &mn_fcn,
            fcn,
            &seed,
            strategy,
            maxfcn,
            edmval,
        );
        // ... check outcome ...
    }
}
```

### 5. MODIFIED: `/src/migrad/mod.rs`

Public API entry point:

```rust
impl MnMigrad {
    // Existing method (unchanged)
    pub fn minimize(&self, fcn: &dyn FCN) -> FunctionMinimum {
        let mn_fcn = MnFcn::new(fcn, &trafo);
        minimizer::VariableMetricMinimizer::minimize(...)
    }
    
    // NEW: Analytical gradient method
    pub fn minimize_grad(&self, fcn: &dyn FCNGradient) -> FunctionMinimum {
        minimizer::VariableMetricMinimizer::minimize_with_gradient(
            fcn,
            &trafo,
            &self.strategy,
            max_fcn,
            self.tolerance,
        )
    }
}
```

### 6. MODIFIED: `/src/minimum/gradient.rs`

Add method to mark gradients:

```rust
impl FunctionGradient {
    // Existing methods unchanged
    
    // NEW: Mark gradient as analytical
    pub fn set_analytical(&mut self, analytical: bool) {
        self.analytical = analytical;
    }
}
```

### 7. NEW: `/tests/migrad_analytical.rs` (180 lines)

Integration tests:

```rust
#[test]
fn migrad_analytical_quadratic() {
    let result = MnMigrad::new()
        .add("x", 3.0, 0.1)
        .add("y", 2.0, 0.1)
        .minimize_grad(&Quadratic);
    // Verify convergence and parameter values
}

#[test]
fn migrad_analytical_vs_numerical_quadratic() {
    // Both paths should reach same minimum
}

#[test]
fn migrad_analytical_with_bounds() {
    // Verify bounded parameters work correctly
}
```

## Call Flow Comparison

### Numerical Gradient Path (Existing)
```
MnMigrad::minimize(fcn: &dyn FCN)
  ↓
VariableMetricMinimizer::minimize()
  ↓
MigradSeedGenerator::generate()
  ├─ InitialGradientCalculator::compute()
  └─ Numerical2PGradientCalculator::compute()  ← Multiple FCN calls
  ↓
VariableMetricBuilder::minimum()
  └─ iterate()
      ├─ Newton step
      ├─ Line search (FCN calls)
      └─ Numerical2PGradientCalculator::compute_with_previous()  ← 2n FCN calls
```

### Analytical Gradient Path (NEW)
```
MnMigrad::minimize_grad(fcn: &dyn FCNGradient)
  ↓
VariableMetricMinimizer::minimize_with_gradient()
  ↓
MigradSeedGenerator::generate_with_gradient()
  └─ AnalyticalGradientCalculator::compute()  ← No FCN calls for gradient
  ↓
VariableMetricBuilder::minimum_with_gradient()
  └─ iterate_with_gradient()
      ├─ Newton step
      ├─ Line search (FCN calls)
      └─ AnalyticalGradientCalculator::compute()  ← No FCN calls for gradient
```

## Summary of Benefits

1. **Reduced FCN Calls:**
   - Numerical: ~2n+1 per iteration (central differences)
   - Analytical: 0 for gradient, only line search FCN calls

2. **Accuracy:**
   - User provides exact gradient
   - No differentiation error accumulation

3. **Backward Compatible:**
   - Existing code continues to work
   - minimize() method unchanged
   - Optional trait (FCNGradient extends FCN)

4. **Seamless Integration:**
   - All other optimizers unchanged
   - Same DFP update
   - Same line search
   - Same convergence criteria
