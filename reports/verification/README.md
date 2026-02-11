# Cross-Implementation Verification (ROOT vs Rust)

Canonical reference target:
- Repo: `https://github.com/root-project/root`
- Subtree: `math/minuit2`
- Tag: `v6-36-08`
- Commit: `a8ca1b23e38d7dbe0ff24027894ca0f2ad65f1bd`

## What is implemented

- ROOT C++ reference runner: `tools/ref_runner_cpp/main.cpp`
- Rust runner: `src/bin/ref_compare_runner.rs`
- Workload set + tolerances: `verification/workloads/root_minuit2_v6_36_08.json`
- Comparator: `scripts/compare_ref_vs_rust.py`
- ROOT build/bootstrap script: `scripts/build_root_reference_runner.sh`
- Traceability matrix generator: `scripts/generate_traceability_matrix.py`
- Traceability gate checker: `scripts/check_traceability_gate.py`
- Claim scorecard generator: `scripts/generate_verification_scorecard.py`

## Reproduce

```bash
scripts/run_full_verification.sh v6-36-08

# Or run components manually:
scripts/build_root_reference_runner.sh v6-36-08
python3 scripts/compare_ref_vs_rust.py
python3 scripts/generate_traceability_matrix.py
python3 scripts/check_traceability_gate.py --mode non-regression
python3 scripts/generate_verification_scorecard.py
```

## Artifacts

- `reports/verification/diff_summary.md`
- `reports/verification/diff_results.csv`
- `reports/verification/raw/ref/*.json`
- `reports/verification/raw/rust/*.json`
- `reports/verification/manifest.json`
- `reports/verification/scorecard.md`
- `reports/verification/known_differences.md`
- `reports/verification/traceability_matrix.csv`
- `reports/verification/traceability_summary.md`
- `verification/traceability/traceability_baseline.csv`
- `verification/traceability/waivers.csv`

## Current snapshot

From latest run in this repo:
- pass: 4
- warn: 2
- fail: 0

Current workload count: 6.

Correctness gates currently pass for all workloads; warnings are NFCN-efficiency deltas only.

This is a real, reproducible differential gate. It currently demonstrates correctness parity on the covered workloads, with remaining call-count efficiency differences.

Traceability status snapshot:
- implemented: 232
- waived: 105
- unresolved: 78

Claim snapshot:
- numerical parity on covered workloads: **YES**
- full 1:1 functional coverage claim: **NO**
- full 100% verifiable coverage claim: **NO**
