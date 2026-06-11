# Goal: j2j fix round 2 — failure-path edges in Hesse verification (bead: minuit2-rs-j2j)

## 1. Objective
Round-1 fixes (F1–F5) verified and stay in the tree (do NOT revert). Second
review found 3 failure-path divergences from ROOT. Fix all 3. ROOT refs =
third_party/root_ref.

## 2. Acceptance Criteria (this round)
- [ ] G1 CONTINUATION BUDGET PROPAGATES TO STATUS: when the Hesse-verified
      continuation runs with the 1.3x budget, the public FunctionMinimum must
      not report reached_call_limit for nfcn in (maxfcn, 1.3*maxfcn] if the
      continuation converged — mirror how ROOT decides call-limit after the
      extension (VariableMetricBuilder.cxx:177-198 and the Minimum() exit
      checks). Add a regression: a fit converging only inside the extension
      window reports valid, not call-limited.
- [ ] G2 HESSE SAG FAILURE RETURNS FAILED STATE: when all sag retries yield
      zero curvature for a parameter, ROOT returns the MnHesseFailed diagonal
      state immediately (MnHesse.cxx — find and cite the exact lines). Our
      src/hesse/calculator.rs:111-113 only breaks the cycle loop, then builds
      off-diagonals from yy[i]=0/stale steps → bogus covariance. Return the
      failure state; Migrad's verification loop must then exit via the
      invalid-Hesse path. Regression: a function flat in one parameter must
      not produce a "valid" covariance through Hesse.
- [ ] G3 INVALID HESSE STATE MUST NOT BECOME THE RESULT: in
      src/migrad/builder.rs:149-151 the invalid hesse_state is pushed then
      returned as states.last() → the public minimum is built FROM the failed
      state. Mirror ROOT: keep the failure in history/diagnostics but build
      the returned minimum from the last usable state (cxx:183-198 — Hesse
      state only enters the returned minimum if error available / call-limit /
      above-EDM), and make sure the minimum is flagged invalid appropriately
      rather than silently exposing parameters-only validity.
- [ ] Regression hold: tests/hahn1_core_parity.rs + hesse strategy-2
      covariance test still pass; differential harness pass=12 warn=0 fail=0;
      NIST plain 9 + hard tier pass; Hahn1 s1 stays OK in the baseline;
      `bash .sc/minuit2-rs-j2j.gate.sh` exit 0; cargo test --all-features
      0 failures; clippy --all-targets clean.

## 3. Verification
`bash .sc/minuit2-rs-j2j.gate.sh`

## 4. Scope
✅ ALWAYS: src/migrad/builder.rs, src/migrad/minimizer.rs (call-limit
   decision), src/hesse/calculator.rs, tests/
⚠️ ASK FIRST: changing FunctionMinimum validity semantics globally;
   any src/minimum/ struct changes
🚫 NEVER: weakening tests/tolerances; reverting round-1 mechanisms;
   dataset special-casing

## 5. Context Pointers
- Round-1 verified loop: src/migrad/builder.rs minimize_with_reseed.
- reports/parity/hahn1_core_divergence.md — extend if mechanisms change.
- Skills: rust.

## 6. Stop Conditions
- DONE when G1–G3 + regression hold pass.
- STOP and report if: G3 requires global validity-semantics changes
  (ask-first); fixing an edge breaks Hahn1 s1 or any differential workload;
  a gate fails twice on the same cause.
