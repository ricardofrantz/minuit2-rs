# Known Differences Ledger (ROOT `v6-36-08` vs `minuit2-rs`)

This ledger records currently observed differences from differential verification runs.

## D-001: NFCN divergence on quadratic fixed-parameter workloads

- Workloads:
  - `quadratic3_fixx_migrad`
  - `quadratic3_fixx_hesse`
- Classification: `algorithmic/path-efficiency`
- Severity: `low` (warning only)
- Expected: `yes`
- Evidence:
  - `reports/verification/diff_summary.md`
  - `reports/verification/diff_results.csv`
- Details:
  - Correctness metrics (fval/edm/params/covariance where applicable) pass tolerances.
  - Function-call counts differ by ~0.51-0.83 relative ratio.
- Disposition: `document`
- Next step:
  - Expand workload set and trend NFCN deltas by algorithm family before deciding if optimization/retuning is necessary.

## D-003: Rosenbrock covariance differs but remains within configured tolerance

- Workloads:
  - `rosenbrock2_migrad`
  - `rosenbrock2_minimize`
- Classification: `algorithmic/path-efficiency`
- Severity: `low` (correctness gate passes with explicit covariance tolerance)
- Expected: `yes`
- Details:
  - Covariance presence now matches and is enforced in the differential gate.
  - Covariance element max abs difference is currently ~0.39 (tolerance: 0.5), consistent with stopping at slightly different nearby points.
- Disposition: `document`
- Next step:
  - Add stricter Rosenbrock convergence workloads (lower tolerance and/or larger call budget) if tighter covariance parity is required.

## Resolved since previous snapshot

- Previous `NoG2CallsWhenFCHasNoG2` P0 gap resolved by adding optional FCN `has_g2()/g2()` and `has_hessian()/hessian()` contract and porting the regression as `tests/root_reference_minuit2.rs::root_no_g2_calls_when_fcn_has_no_g2`.
- Previous `quadratic2_simplex` correctness fail resolved after aligning Simplex EDM tolerance semantics with ROOT (`minedm = tolerance * Up`).
- Previous `rosenbrock2_minimize` correctness fail resolved after aligning `MnMinimize` flow with ROOT combined minimizer logic and propagating covariance from final minimum state.
