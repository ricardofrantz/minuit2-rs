# Phase 3: Migrad — Variable Metric Minimizer (4-6 weeks)

The main workhorse. Most complex phase. This is the core value of the crate.

## Files to Translate

### Core iteration loop
| C++ | → Rust | LOC |
|-----|--------|-----|
| [VariableMetricBuilder.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/VariableMetricBuilder.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/VariableMetricBuilder.cxx) | `migrad/builder.rs` | 379 |
| [VariableMetricEDMEstimator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/VariableMetricEDMEstimator.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/VariableMetricEDMEstimator.cxx) | `migrad/edm.rs` | ~50 |
| [VariableMetricMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/VariableMetricMinimizer.h) | `migrad/minimizer.rs` | ~30 |
| [MnMigrad.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnMigrad.h) | `migrad/mod.rs` | 134 |

### Gradient calculators
| C++ | → Rust | LOC | Notes |
|-----|--------|-----|-------|
| [InitialGradientCalculator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/InitialGradientCalculator.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/InitialGradientCalculator.cxx) | `gradient/initial.rs` | ~150 | First gradient from param steps |
| [Numerical2PGradientCalculator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/Numerical2PGradientCalculator.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/Numerical2PGradientCalculator.cxx) | `gradient/numerical.rs` | ~200 | Forward/central finite diff |
| [HessianGradientCalculator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/HessianGradientCalculator.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/HessianGradientCalculator.cxx) | `gradient/hessian.rs` | ~200 | Gradient from Hessian |
| [AnalyticalGradientCalculator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/AnalyticalGradientCalculator.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/AnalyticalGradientCalculator.cxx) | `gradient/analytical.rs` | ~100 | User-provided gradient |

### Line search
| C++ | → Rust | LOC |
|-----|--------|-----|
| [MnLineSearch.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnLineSearch.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnLineSearch.cxx) | `linesearch.rs` | ~150 |
| [MnParabola.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnParabola.h) | `parabola.rs` | ~30 |
| [MnParabolaFactory.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnParabolaFactory.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnParabolaFactory.cxx) | `parabola.rs` | ~100 |
| [MnParabolaPoint.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnParabolaPoint.h) | `parabola.rs` | ~20 |

### Hessian update
| C++ | → Rust | LOC | Notes |
|-----|--------|-----|-------|
| [DavidonErrorUpdator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/DavidonErrorUpdator.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/DavidonErrorUpdator.cxx) | `updator/davidon.rs` | ~100 | Default: DFP formula |
| [BFGSErrorUpdator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BFGSErrorUpdator.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/BFGSErrorUpdator.cxx) | `updator/bfgs.rs` | ~100 | Alternative |

### Seed & helpers
| C++ | → Rust | LOC | Notes |
|-----|--------|-----|-------|
| [MnSeedGenerator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnSeedGenerator.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnSeedGenerator.cxx) | `migrad/seed.rs` | ~200 | Initial Hessian from gradient |
| [NegativeG2LineSearch.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/NegativeG2LineSearch.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/NegativeG2LineSearch.cxx) | `negative_g2.rs` | ~100 | Handles negative 2nd derivatives |
| [MnPosDef.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnPosDef.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnPosDef.cxx) | `posdef.rs` | ~80 | Force positive-definite Hessian |
| [MnEigen.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnEigen.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnEigen.cxx) | `eigen.rs` | ~50 | Eigenvalue calculation |
| [mnteigen.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mnteigen.cxx) | `eigen.rs` | ~100 | Tridiagonal eigenvalue routine |

**Total**: ~2,300 LOC of C++ to translate.

## Algorithm

```
1. Compute initial gradient g₀ by finite differences
2. Build initial inverse Hessian H₀ (diagonal from gradient steps)
3. Loop:
   a. direction = -H * g
   b. step = line_search(direction)    ← parabolic interpolation
   c. x_new = x + step * direction
   d. g_new = numerical_gradient(x_new)
   e. H_new = DFP_update(H, g_new - g, step * direction)
   f. EDM = g_new^T * H_new * g_new
   g. if EDM < tolerance → converged
   h. if iterations > max → abort
   i. if H not positive-definite → force posdef, retry
4. Return minimum + covariance (H = approx inverse Hessian)
```

## Critical Numerical Details

- **EDM** = `g^T * H^{-1} * g` — primary convergence criterion
- **DFP update**: `H' = H + (dx * dx^T) / (dx^T * dg) - (H * dg * dg^T * H) / (dg^T * H * dg)`
- **Positive-definiteness**: if eigenvalues of H go negative after update, reset H to diagonal
- **Gradient step size**: Strategy 0 = forward diff, Strategy 2 = central diff (2x FCN evals)
- **Line search**: NOT Wolfe conditions — Minuit uses parabolic interpolation on 3 points

## Translation Order

1. `MnParabola` + `MnParabolaFactory` — pure math, no deps
2. `MnLineSearch` — depends on parabola
3. `InitialGradientCalculator` → `Numerical2PGradientCalculator`
4. `DavidonErrorUpdator` → `BFGSErrorUpdator`
5. `MnPosDef` + `MnEigen` + `NegativeG2LineSearch`
6. `MnSeedGenerator` — uses all the above
7. `VariableMetricEDMEstimator`
8. `VariableMetricBuilder` — the main loop, depends on everything
9. `VariableMetricMinimizer` + `MnMigrad` — thin wiring

## Validation

Compare iteration-by-iteration against iminuit (set `print_level=3` in both):
```python
from iminuit import Minuit
m = Minuit(fcn, x=2, y=2)
m.print_level = 3
m.migrad()
# Compare: m.values, m.errors, m.fval, m.nfcn, m.covariance
```

### Test cases
- [ ] Rosenbrock 2D — verify iterations & fval match C++
- [ ] Wood's function (4 params) — standard Minuit test
- [ ] Powell's function — tests Hessian update edge cases
- [ ] Gaussian fit (3 params) — practical physics case
- [ ] 10-param fit — stress test for large Hessian
- [ ] Bounded params — verify transform + Migrad integration
- [ ] User-provided gradient — test `AnalyticalGradientCalculator`

## Risks

1. **`VariableMetricBuilder.cxx` (379 lines)** — most critical file. Every conditional is a hard-won stability heuristic. Translate line-by-line, not "idiomatically".
2. **Gradient accuracy** — finite-difference step sizes must match Minuit2 exactly or convergence breaks.
3. **Parabolic line search** is custom, not textbook Armijo/Wolfe. Don't substitute.
