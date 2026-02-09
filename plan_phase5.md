# Phase 5: CLI, Benchmarks, Publish (2-3 weeks)

Polish. Prove correctness. Ship.

## Files to Translate (stretch)

### Combined minimizer (Simplex → Migrad pipeline)
| C++ | → Rust | LOC |
|-----|--------|-----|
| [MnMinimize.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnMinimize.h) | `combined/mod.rs` | ~50 |
| [CombinedMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/CombinedMinimizer.h) | `combined/minimizer.rs` | ~30 |
| [CombinedMinimumBuilder.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/CombinedMinimumBuilder.h) + [.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/CombinedMinimumBuilder.cxx) | `combined/builder.rs` | ~100 |

## Deliverables

### Examples
- [ ] `examples/rosenbrock.rs` — classic 2D optimization
- [ ] `examples/gaussian_fit.rs` — fit Gaussian to noisy data with Hesse+Minos
- [ ] `examples/chi_square.rs` — physics chi-square with error propagation

### Benchmarks (`criterion`)
- [ ] Rosenbrock 2D, 10D, 50D
- [ ] Gaussian fit (3 params)
- [ ] Multi-peak fit (10+ params)
- [ ] Report: iterations, FCN evaluations, wall time, final fval
- [ ] Verify numerical agreement with iminuit (values + errors within tolerance)

### Publish
- [ ] `cargo publish` to crates.io as `minuit2-rs` v0.1.0
- [ ] API docs (`#[doc]`) on all public types
- [ ] Announce: r/rust, r/Physics, Scikit-HEP forums

### Stretch
- [ ] `rayon`-parallel parameter scans (feature-gated)
- [ ] PyO3 Python bindings
- [ ] User-provided analytic gradient support
- [ ] Combined minimizer (Simplex → Migrad)

## Success Criteria
- All iminuit reference fits reproduce (values, errors match)
- Performance within 2x of C++ Minuit2 (ideally parity)
- `cargo clippy` clean, zero `unsafe`
- Installable via `cargo add minuit2-rs`
