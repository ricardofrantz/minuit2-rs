# Goal: j2j fix round — Hesse-verification loop parity edges (bead: minuit2-rs-j2j)

## 1. Objective
Your j2j Hahn1 fix is in the tree (do NOT revert). Review confirmed the
mechanism, but the new Hesse-verification loop in src/migrad/builder.rs
diverges from ROOT in 5 control-flow edge cases. Fix all 5; original ACs
(goal commit daac3b2) still apply. All ROOT refs = third_party/root_ref.

## 2. Acceptance Criteria (this round)
- [ ] F1 Hesse runs on the strategy condition, not only on nominal
      convergence: ROOT enters the Hesse block after each inner pass whenever
      `strategy>=2 || (strategy==1 && dcovar>0.05)` — even when the pass
      exited above tolerance (VariableMetricBuilder.cxx:130-160). Currently
      `if !must_continue && should_hesse(last)` skips Hesse for
      above-tolerance exits.
- [ ] F2 Hesse-verified state is ALWAYS appended (ROOT AddResult before the
      validity/EDM tests, cxx:134-145) so the returned FunctionMinimum carries
      the Hesse covariance/EDM, not the pre-Hesse one — including when
      verification succeeds and no continuation happens. Add a test: for a
      strategy-2 (or dcovar>0.05 strategy-1) fit, the returned covariance
      matches an explicit MnHesse run on the same point.
- [ ] F3 Invalid Hesse result exits the loop (ROOT breaks on
      `!st.IsValid()`, cxx:144-149, where IsValid includes Error().IsValid(),
      MinimumState.h:67-75). Use the Hesse result/error validity flags — our
      MinimumState::is_valid() is parameters-only, so the current guard can
      continue from a broken error matrix.
- [ ] F4 Hesse verification uses the ORIGINAL maxfcn; only the continuation
      pass gets the 1.3x budget (cxx:138-140, 177-180).
- [ ] F5 Negative/NaN EDM guard right after the EDM estimate in the inner
      loop: ROOT checks isnan(edm) and edm<0, runs MnPosDef and re-estimates,
      aborts if still negative (cxx:301-322). Needed now that negative-delgam
      Davidon updates are applied.
- [ ] Regression hold: tests/hahn1_core_parity.rs still passes; differential
      harness pass=12 warn=0 fail=0; NIST plain 9 passed + hard tier passes;
      nist_hard_baseline.md regenerated if statuses move (Hahn1 s1 must stay
      OK); `bash .sc/minuit2-rs-j2j.gate.sh` exit 0; cargo test
      --all-features 0 failures; clippy --all-targets clean.

## 3. Verification
`bash .sc/minuit2-rs-j2j.gate.sh`

## 4. Scope
✅ ALWAYS: src/migrad/builder.rs, src/minimum/ (validity flags if F3 needs
   them), src/hesse/ (result flags read-only wiring), tests/, reports/parity/
⚠️ ASK FIRST: changing MinimumState::is_valid() semantics globally (other
   call sites depend on it — prefer a local error-validity check in builder)
🚫 NEVER: weakening tests/tolerances; dataset special-casing; reverting the
   landed Hahn1 mechanisms

## 5. Context Pointers
- Review findings (verbatim ROOT citations): .sc/minuit2-rs-j2j.review.out
  is large — the 5 findings are restated fully above; trust the cited lines,
  read them in third_party/root_ref.
- reports/parity/hahn1_core_divergence.md — update it if the fixes change
  the documented mechanism/numbers.
- Skills: rust.

## 6. Stop Conditions
- DONE when F1–F5 + regression hold all pass.
- STOP and report if: fixing an edge breaks Hahn1 s1 certification or any
  differential workload (report which F and the numbers); F3 requires the
  global is_valid() change (ask-first); a gate fails twice on the same cause.
