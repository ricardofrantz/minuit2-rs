# `minuit2-rs` — Pure Rust Port of Standalone Minuit2

Minuit2 is CERN's parameter optimization engine, the standard in high-energy physics since 1975.  
This crate is a **pure Rust port of ROOT Minuit2**, specifically the `math/minuit2` subsystem from:
- `https://github.com/root-project/root/tree/master/math/minuit2`

Target compatibility baseline for verification work:
- ROOT release tag: `v6-36-08`
- Scope: **Minuit2 only** (`math/minuit2`), not the broader ROOT framework layers.

Compatibility snapshot (2026-02-11, differential harness):
- pass: `10`
- warn: `2`
- fail: `0`
- Details: `reports/verification/diff_summary.md`
- Claim scorecard: `reports/verification/scorecard.md`
- Legacy C++ executed-surface coverage: `reports/verification/reference_coverage/summary.md`

Verification snapshot (2026-02-11, ROOT `v6-36-08` baseline):
- Differential workloads: `12` (`pass=10`, `warn=2`, `fail=0`)
- Traceability matrix: `415` symbols (`implemented=303`, `waived=112`, `unresolved=0`)
- Rust line coverage: `73.12%` (`--no-default-features`), `69.51%` (`--all-features`)
- Reference C++ executed-surface coverage: `618 / 1840` functions (`33.59%`) and `48.96%` file line coverage
- Executed-surface unmapped gaps: `P0=0`, `P1=48`, `P2=425` (`strict gate: FAIL`, non-regression gate: PASS)

Verification claim status:
- We **can** claim numerical parity on the covered differential workloads (`fail=0`).
- We **can** claim zero known P0 gaps in the current parity/traceability gates.
- We **cannot** currently claim full 1:1 functional coverage or “100% verifiable coverage”.
  Current blocker: executed-surface strict gate requires `P0=0` and `P1=0`, but current `P1=48`.

Recent verification burn-down commits:
- `1306127` — expanded verification workloads and added executed-surface gate.
- `15cada5` — reduced executed-surface false positives and rebaselined gaps.
- `3de1c79` — burned down matrix-heavy executed-surface P1 gaps.
- `93b45c2` — tightened executed-surface mapping and reduced P1 to 48.

One-command reproducible verification (ROOT `v6-36-08` baseline):

```bash
scripts/run_full_verification.sh v6-36-08
```

## Verification Guide For Future Developers

This section is intended to be operational, not marketing. It documents what is proven today, what is still open, and how to continue burn-down work safely.

What is currently high-confidence:
- Numerical correctness on the covered workload suite: `reports/verification/diff_summary.md` shows `fail=0` across 12 ROOT-vs-Rust differential workloads.
- Symbol-level parity gate is clean: `reports/verification/traceability_summary.md` shows `unresolved=0`.
- No known high-severity parity regressions in claim gates: `reports/verification/scorecard.md` reports ROOT P0 gaps `0`.

What is currently not perfect:
- Executed-surface strict completeness is not met. `reports/verification/executed_surface_mapping.md` currently reports `P0=0`, `P1=48`, `P2=425`, so strict gate (`P0==0 && P1==0`) fails.
- Differential warnings remain in NFCN divergence for:
  - `quadratic3_fixx_migrad`
  - `quadratic3_fixx_hesse`
- Benchmark evidence does not show blanket speedups. `reports/benchmarks/benchmark_baseline.md` shows scan-parallel overhead on small scans (`0.07x` speedup ratio for the benchmarked case).

How to interpret executed-surface priorities:
- `P0`: unresolved/high-risk mapping issue; treat as release-blocking regression.
- `P1`: missing mapping for executed upstream functions; typically alias drift, API-shape drift, or incomplete parity mapping.
- `P2`: lower-priority/waived categories (intentional architectural replacements, out-of-scope, or known tooling limitations).

Why some large gap files are not immediate blockers:
- `inc/Minuit2/MnPrint.h` and `src/MnPrint.cxx`: logging architecture was intentionally reshaped in Rust.
- `inc/Minuit2/MnMatrix.h` and `src/MnMatrix.cxx`: matrix kernels are represented via `nalgebra`-backed Rust structures instead of 1:1 symbol clones.
- `inc/ROOT/span.hxx` and `src/MPIProcess.*`: treated as architectural/out-of-scope in current waiver policy.

Current `P1` concentration (use this to prioritize burn-down):
- `inc/Minuit2/MinimumSeed.h` (`5`)
- `inc/Minuit2/FunctionGradient.h` (`4`)
- `inc/Minuit2/MnUserParameterState.h` + `src/MnUserParameterState.cxx` (`8` combined)
- `inc/Minuit2/MnUserParameters.h` + `src/MnUserParameters.cxx` (`5` combined)
- `inc/Minuit2/MnUserTransformation.h` + `src/MnUserTransformation.cxx` (`6` combined)

Recommended burn-down workflow:
1. Run `scripts/run_full_verification.sh v6-36-08` and confirm non-regression gates pass before editing mappings.
2. Inspect `reports/verification/executed_surface_gaps.csv`, sorted by `gap_priority` then `call_count`.
3. For true API equivalences, add mapping/alias improvements in the parity + traceability pipeline, not ad-hoc waivers.
4. For intentional design differences, add explicit waiver rules with rationale in `verification/traceability/waiver_rules.csv`.
5. Re-run executed-surface generation and gate checks:
   - `python3 scripts/generate_executed_surface_mapping.py`
   - `python3 scripts/check_executed_surface_gate.py --mode non-regression`
   - `python3 scripts/check_executed_surface_gate.py --mode strict`
6. Only update `verification/traceability/executed_surface_gaps_baseline.csv` after review of why deltas are valid.

Hard claim boundary (do not overstate):
- Until executed-surface strict gate is green (`P0=0`, `P1=0`) and differential warnings are resolved, do not claim full 1:1 functional coverage or 100% verifiable equivalence.

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
- `parallel`: Enables `rayon` support for `MnScan::scan_parallel`.

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
| **MnScan** | Done | 1D parameter scans (`scan_parallel` available with `parallel` feature) |
| **MnContours** | Done | 2D confidence contours |

## Robustness & Security

`minuit2-rs` is built for reliability in scientific computing:
- **Zero Unsafe.** The entire codebase is written in 100% safe Rust.
- **Numerical Resilience.** Gracefully handles `NaN` and `Inf` by treating them as high-value penalties, preventing optimizer crashes.
- **Stress Tested.** Validated against 50-parameter problems and rugged landscapes like the **Goldstein-Price** function.

## Upstream Source

[ROOT Minuit2 (`math/minuit2`)](https://github.com/root-project/root/tree/master/math/minuit2).
This port replaces custom C++ linear algebra with `nalgebra` and manual memory management with Rust's ownership model.

## License

MIT OR Apache-2.0
