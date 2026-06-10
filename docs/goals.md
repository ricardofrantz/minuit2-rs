# Goal: NIST hard-dataset baseline (iminuit vs minuit2-rs)   (bead: minuit2-rs-aic)

## 1. Objective
Measure what upstream iminuit (its own vendored C++ Minuit2) actually does
from NIST "Start 2" on the 4 unsolved StRD datasets (Lanczos3, BoxBOD, MGH09,
Hahn1), side by side with minuit2-rs. This defines the honest pass bar for the
multistart-recipe bead: parity vs genuine gap. (VISION: claims backed by
evidence; "solves the hard problems the original solves".)

## 2. Acceptance Criteria
- [ ] `scripts/nist_hard_baseline.py` (new) runs all 4 datasets through BOTH
      iminuit.Minuit and minuit2.Minuit from Start 2, strategies 1 AND 2,
      errordef=1, recording per run: valid flag, fval vs certified residual
      SS, params within NIST tolerance (same tolerance scheme as
      tests/nist_strd_certified.rs), NFCN.
- [ ] Model residuals + Start 2 + certified values are read from ONE shared
      Python module (new, e.g. python/compat/nist_models.py) — NOT duplicated
      formulas; datasets parsed from examples/data/nist/*.dat where present
      (download via scripts/fetch_scientific_demo_data.sh conventions if a
      .dat is missing — ⚠️ ask first before adding new data files).
- [ ] `reports/parity/nist_hard_baseline.md` (new): 4-row result matrix
      (columns: iminuit s1, iminuit s2, minuit2-rs s1, minuit2-rs s2), plus a
      one-paragraph conclusion per dataset: parity-failure (both fail) or
      genuine gap (iminuit succeeds, we fail).
- [ ] Any genuine-gap dataset is flagged at the top of the report as the
      priority target for the recipe bead.
- [ ] Script is deterministic and runs in <5 min.

## 3. Verification
- Env: python3 -m venv .venv-maturin (if missing) && . .venv-maturin/bin/activate &&
  pip install maturin iminuit numpy pytest &&
  PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
- Quick: . .venv-maturin/bin/activate && python scripts/nist_hard_baseline.py --dataset BoxBOD 2>&1 | tail -15
- Full (logged): python scripts/nist_hard_baseline.py 2>&1 | tail -40 ; then the report file exists and matches the printed matrix.

## 4. Scope
✅ ALWAYS: scripts/nist_hard_baseline.py (new), python/compat/nist_models.py
   (new), reports/parity/nist_hard_baseline.md (new).
⚠️ ASK FIRST: new data files under examples/data/; changes to
   tests/nist_strd_certified.rs; pip packages beyond maturin/iminuit/numpy/pytest.
🚫 NEVER: src/ (no Rust changes this cycle), README.md, CHANGELOG.md,
   .github/, existing scripts.

## 5. Non-Goals / Constraints
- NOT this cycle: the multistart recipe itself (bead minuit2-rs-7qf); fixing
  any divergence found.
- Read-only comparison: do not tweak tolerances/strategies beyond the matrix.
- ROOT-runner column is optional; iminuit IS the C++-backed comparator. Skip
  ROOT if wiring it costs more than ~30 min — note the skip in the report.

## 6. Context Pointers
- `br show minuit2-rs-aic` (incl. comments — venv setup note); `VISION.md`.
- Start 2 values + certified params + tolerances: tests/nist_strd_certified.rs
  (port them into nist_models.py verbatim, cite NIST dataset pages).
- Data: examples/data/nist/ + examples/data/SHA256SUMS;
  harness style: python/compat/diff_iminuit.py.
- Skills: python.

## 7. Task Breakdown
1. nist_models.py with the 4 models (residual fn, start2, certified, tol).
2. Baseline script: run matrix, print + write report.
3. Run all 4; write conclusions; flag genuine gaps.

## 8. Stop Conditions
- DONE when all Acceptance Criteria pass.
- STOP and report if: iminuit cannot be installed in the venv, a dataset's
  .dat file is missing (⚠️ gate), or results are non-deterministic across
  two consecutive runs.
