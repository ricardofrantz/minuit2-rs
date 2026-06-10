# Goal: Resolve quadratic3_fixx NFCN warnings — verify Rust is not under-iterating   (bead: minuit2-rs-b9e)

## 1. Objective
The last two WARN rows in the ROOT differential gate: quadratic3_fixx_migrad
(nfcn rel 0.828, Rust 5 vs ROOT 29) and quadratic3_fixx_hesse (0.513, 19 vs
39). Rust is suspiciously CHEAP on a fixed-parameter fit. Establish exactly
which evaluations ROOT performs that Rust skips, then either (a) implement
the missing work if it affects result quality, or (b) document the
intentional divergence via an explicit per-workload waiver so the gate ends
at warn=0. Never relax the 0.5 warn threshold globally.

## 2. Diagnosis head start (verified in prior cycles)
- Trace diff: divergence at ITERATION 0 — seed phase (fixx Rust nfcn 1 vs
  ROOT 7). Traces: reports/verification/raw/trace/quadratic3_fixx_migrad.*;
  regenerate via feature `trace` + MINUIT2_RS_TRACE_JSONL +
  scripts/diff_iteration_traces.py.
- The 9ma cycle (just merged) rewrote src/gradient/numerical.rs to mirror
  ROOT's gradient calculator — fixx warns survived it, so look at the seed
  gradient over fixed slots, Hesse sweeps, covariance squeeze.
- fixx_migrad cov rel diff 3.9e-4 is the largest among passers — check
  whether the skipped evaluations explain it.
- Diagnose migrad first; hesse likely inherits.

## 3. Acceptance Criteria
- [ ] reports/verification/diff_summary.md ends at fail=0 warn=0 — via
      matched NFCN (route a) or an explicit documented per-workload waiver
      (route b), not a global threshold change.
- [ ] Written explanation of WHAT the C++ extra evaluations are (waiver
      rationale or README dev-guide note), citing ROOT v6-36-08 source.
- [ ] If route (a): cov rel diff on fixx_migrad improves or is explained.
- [ ] cargo test --all-features green; no other workload's status degrades
      (rosenbrock2 rows must keep their new ≤0.10 nfcn_rel).

## 4. Verification
- Quick: export PATH="$HOME/.cargo/bin:$PATH" && cargo test --test root_reference_minuit2 --test hesse --test root_reference_covariance 2>&1 | tail -20
- Full (logged): scripts/run_full_verification.sh v6-36-08 2>&1 | tail -30
  then status counts in diff_results.csv: pass=12 warn=0 fail=0 (or pass=10
  + 2 explained-waived). NOTE: the script's cargo-llvm-cov coverage tier is
  known-missing on this box — skip/ignore that step, it is not this gate.

## 5. Scope
✅ ALWAYS: src/migrad/seed.rs, src/hesse/calculator.rs,
   src/covariance_squeeze.rs (as diagnosis dictates), the diff/waiver
   tooling config it requires (e.g. scripts/compare_ref_vs_rust.py waiver
   input or verification/traceability/ waiver rules), reports/verification/*
   (regenerated), README.md (known-gaps note), tests/ (regression pin).
⚠️ ASK FIRST: src/gradient/numerical.rs (just rewritten — don't churn it),
   src/migrad/builder.rs, any global threshold or tolerance.
🚫 NEVER: CHANGELOG.md, .github/, python/.

## 6. Context Pointers
- br show minuit2-rs-b9e (full background + route definitions); VISION.md.
- Existing waiver style: verification/traceability/ + reports/verification/known_differences.md.

## 7. Stop Conditions
- DONE when all Acceptance Criteria pass.
- STOP and report BLOCKED if: the fix needs an ⚠️ Ask-first file, route
  (a)-vs-(b) is genuinely ambiguous after diagnosis (state the evidence both
  ways), or any previously passing workload regresses.
