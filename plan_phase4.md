# Phase 4: Error Analysis — Hesse, Minos, Scan, Contours (3-4 weeks)

What separates Minuit from generic optimizers. The reason physicists use it.

## Files to Translate

### Hesse (symmetric errors from full Hessian)
| C++ | → Rust | LOC |
|-----|--------|-----|
| [MnHesse.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnHesse.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnHesse.cxx) | `hesse.rs` | 418 |
| [MnCovarianceSqueeze.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnCovarianceSqueeze.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnCovarianceSqueeze.cxx) | `covariance_squeeze.rs` | ~100 |
| [MnGlobalCorrelationCoeff.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnGlobalCorrelationCoeff.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnGlobalCorrelationCoeff.cxx) | `global_cc.rs` | ~80 |

### Minos (asymmetric errors by profile likelihood)
| C++ | → Rust | LOC |
|-----|--------|-----|
| [MnMinos.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnMinos.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnMinos.cxx) | `minos.rs` | ~300 |
| [MinosError.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinosError.h) | `minos_error.rs` | ~80 |
| [MnCross.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnCross.h) | `cross.rs` | ~50 |
| [MnFunctionCross.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnFunctionCross.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnFunctionCross.cxx) | `function_cross.rs` | ~200 |

### Scan & Contours
| C++ | → Rust | LOC |
|-----|--------|-----|
| [MnParameterScan.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnParameterScan.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnParameterScan.cxx) | `scan/parameter_scan.rs` | ~100 |
| [MnScan.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnScan.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnScan.cxx) | `scan/mod.rs` | ~100 |
| [ScanBuilder.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ScanBuilder.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/ScanBuilder.cxx) | `scan/builder.rs` | ~80 |
| [ScanMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ScanMinimizer.h) | `scan/minimizer.rs` | ~30 |
| [MnContours.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnContours.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnContours.cxx) | `contours.rs` | ~200 |
| [ContoursError.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ContoursError.h) | `contours.rs` | ~50 |

**Total**: ~1,800 LOC of C++ to translate.

## What Each Does

**Hesse**: Compute full d²f/dx_i dx_j by finite differences → invert → covariance matrix → symmetric errors. The `MnCovarianceSqueeze` removes fixed parameters from the matrix. `MnGlobalCorrelationCoeff` extracts correlation coefficients.

**Minos**: For parameter i, fix it at values above/below minimum, re-minimize all others, find where f increases by `error_def`. Runs **full Migrad per scan point** — expensive but gives true profile likelihood errors. `MnFunctionCross` does the crossing-point search.

**Scan**: Evaluate f along one parameter axis. `MnParameterScan` is the low-level evaluator, `MnScan`/`ScanBuilder` wrap it as a minimizer.

**Contours**: Trace the f = f_min + error_def contour in a 2-param plane. Uses `MnFunctionCross` internally.

## Translation Order

1. `MnGlobalCorrelationCoeff` — pure math on covariance matrix
2. `MnCovarianceSqueeze` — matrix manipulation
3. `MnHesse` — depends on gradient calculator from Phase 3
4. `MnCross` + `MinosError` — result types
5. `MnFunctionCross` — crossing-point finder (used by both Minos and Contours)
6. `MnMinos` — depends on Migrad + MnFunctionCross
7. `MnParameterScan` → `ScanBuilder` → `MnScan`
8. `MnContours` — depends on MnFunctionCross

## Validation

```python
from iminuit import Minuit
m = Minuit(fcn, x=2, y=2)
m.migrad()
m.hesse()   # Compare: m.errors, m.covariance, m.valid
m.minos()   # Compare: m.merrors (upper/lower per param)
```

### Test cases
- [ ] Gaussian fit — Hesse errors should equal Minos errors (symmetric case)
- [ ] Asymmetric likelihood — Minos upper != lower, Hesse gives average
- [ ] Bounded parameters — Hesse near limits gives singular matrix warning
- [ ] 2D contour — verify contour points match iminuit
- [ ] 1D scan — profile should show parabolic shape at minimum

## Risks

- Minos depends on robust Migrad — any Phase 3 bugs surface here as wrong errors.
- `MnFunctionCross` has complex convergence logic for finding the crossing point.
- Singular Hessian near parameter bounds needs graceful degradation (warn, don't crash).
