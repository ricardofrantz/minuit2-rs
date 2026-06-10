# Goal: Hahn1 core divergence — diagnose, then fix with ROOT citation (bead: minuit2-rs-j2j)

## 1. Objective
iminuit (C++ Minuit2) reaches the certified Hahn1 solution from NIST Start 1
(fval=1.53244, nfcn=581, strategy 1); minuit2-rs returns valid=False on BOTH
strategies (s1 nfcn=72 fval=51623; s2 nfcn=260 fval=46950). Since iminuit IS
ROOT Minuit2, a divergence this large is a parity gap somewhere in our chain.
Diagnose the FIRST divergence point, then fix it if the mechanism is citable
to ROOT source. Diagnosis before any fix — no tuning by trial.

## 2. Acceptance Criteria
- [ ] AC1 DIAGNOSIS: reports/parity/hahn1_core_divergence.md states (a) WHY
      valid=False — which FunctionMinimum flag fires (above_max_edm? not
      posdef? call limit?) at which iteration; (b) the first point where
      minuit2-rs's trajectory diverges from iminuit's (seed? first gradient?
      iteration N?) with side-by-side numbers; (c) the mechanism, citing the
      ROOT Minuit2 file:line whose behavior we miss. Useful probes:
      scripts/nist_hard_baseline.py (env: `. .venv-maturin/bin/activate`),
      iminuit's per-iteration state via callbacks/prints, and our own trace
      hooks (see scripts/diff_iteration_traces.py).
- [ ] AC2 FIX (only if AC1 yields a ROOT-citable gap): implement the parity
      fix in src/ with the ROOT file:line in comments; add a regression test
      that FAILS pre-change (state the command proving it). Success target:
      Hahn1 minuit2-rs reaches certified params from NIST Start 1 or 2 at the
      baseline's 1e-2 tolerance — or, if it still cannot, prove iminuit's
      success depends on behavior we correctly lack and STOP (that is AC1
      output, not a failure).
- [ ] AC3 NO REGRESSIONS: differential harness pass=12 warn=0 fail=0;
      plain NIST tier 9 passed; hard tier (--ignored) still certifies;
      regenerate reports/parity/nist_hard_baseline.md if any status changed.
- [ ] AC4 GATES: `bash .sc/minuit2-rs-j2j.gate.sh` exit 0;
      `cargo test --all-features` 0 failures; clippy --all-targets clean.

## 3. Verification
`bash .sc/minuit2-rs-j2j.gate.sh`  (tests, clippy, differential harness,
maturin rebuild + baseline rerun, NIST plain+hard tiers)

## 4. Scope
✅ ALWAYS: reports/parity/, tests/, scripts/ probe additions,
   src/ ONLY for a fix whose mechanism cites ROOT Minuit2 source
⚠️ ASK FIRST: changing any convergence/tolerance constant without a ROOT
   citation; touching verification/workloads/*.json (waivers); python/minuit2
   binding semantics
🚫 NEVER: weakening existing tests or tolerances; "fixing" Hahn1 by special-
   casing the dataset or seeding from certified values

## 5. Context Pointers
- `br show minuit2-rs-j2j`; reports/parity/nist_hard_baseline.md (current rows)
- Prior parity work: k5h = seed NegativeG2LineSearch (did NOT move Hahn1),
  9ma = Numerical2PGradientCalculator parity. Suspect remaining areas: MnPosDef
  in-iteration handling, line search (MnLineSearch.cxx), error updator details,
  MnFcn call accounting — nfcn=72 for 7 params suggests early bailout.
- Hahn1: 7-param rational poly, x in [60, 1000] — severe scaling; certified
  fval=1.53244. Datasets parsed by python/compat/nist_models.py.
- Skills: rust.

## 6. Stop Conditions
- DONE when AC1 + AC3 + AC4 pass, and AC2 either lands or is justified out.
- STOP and report if: the mechanism is found but the fix touches an ask-first
  item; no ROOT-citable mechanism after the trace comparison (report the
  side-by-side trace as the finding); a gate fails twice on the same cause.
