# Phase 2: Simplex Minimizer (3-4 weeks)

First working minimizer. Derivative-free Nelder-Mead. Also useful later as pre-conditioner for Migrad.

## Files to Translate

| C++ | → Rust | LOC | Notes |
|-----|--------|-----|-------|
| [SimplexSeedGenerator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SimplexSeedGenerator.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SimplexSeedGenerator.cxx) | `simplex/seed.rs` | ~100 | Build initial N+1 simplex from param steps |
| [SimplexParameters.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SimplexParameters.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SimplexParameters.cxx) | `simplex/parameters.rs` | ~80 | Vertex storage + sorting |
| [SimplexBuilder.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SimplexBuilder.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SimplexBuilder.cxx) | `simplex/builder.rs` | ~228 | Main iteration: reflect/expand/contract |
| [SimplexMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SimplexMinimizer.h) | `simplex/minimizer.rs` | ~30 | Composes seed+builder |
| [MnSimplex.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnSimplex.h) | `simplex/mod.rs` | ~97 | Public API (extends MnApplication) |

**Total**: ~535 LOC of C++ to translate.

## Algorithm

```
1. Construct initial simplex: N+1 vertices, each offset along one parameter axis
2. Evaluate FCN at all vertices
3. Loop:
   a. Sort vertices by function value (best → worst)
   b. Compute centroid of all vertices except worst
   c. Reflect worst through centroid
   d. If reflected < best → try expansion
   e. If reflected > second-worst → try contraction
   f. If contraction fails → shrink all toward best
   g. Check convergence: EDM or simplex size < tolerance
4. Return best vertex as minimum
```

## Translation Order

1. `SimplexParameters` — just a `Vec<(f64, Vec<f64>)>` with sort
2. `SimplexSeedGenerator` — creates initial vertices
3. `SimplexBuilder` — the algorithm loop (depends on Phase 1 framework)
4. `SimplexMinimizer` + `MnSimplex` — thin wiring

## Validation

Run identical problems in iminuit and compare:
```python
from iminuit import Minuit
m = Minuit(rosenbrock, x=0, y=0)
m.simplex()
# Compare: m.values, m.fval, m.nfcn
```

### Test cases
- [ ] Rosenbrock 2D (classic)
- [ ] Quadratic bowl (trivial, should converge in ~N iterations)
- [ ] Gaussian fit to noisy data (3 params: amp, mean, sigma)
- [ ] Bounded parameter (verify transforms work with simplex)

## Risks
- `SimplexBuilder` convergence criteria use Minuit-specific EDM heuristics, not textbook Nelder-Mead stopping rules. Match the C++ exactly.
- `MnApplication::minimize()` (from Phase 1) drives the loop — must be solid.
