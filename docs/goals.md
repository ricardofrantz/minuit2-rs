# Goal: k5h fix round — close 3 review findings on NegativeG2LineSearch parity (bead: minuit2-rs-k5h)

## 1. Objective
Your previous k5h implementation is in the working tree (do NOT revert it).
External review confirmed 3 parity defects against pinned ROOT v6-36-08
(third_party/root_ref). Fix them; the original ACs (goal commit 793bcbe) still apply.

## 2. Acceptance Criteria (this round)
- [ ] F1 Zero-gradient step direction: src/migrad/seed.rs ~L213 uses `+gstep` when grad == 0; ROOT's else-branch (NegativeG2LineSearch.cxx:80-83) uses `-Gstep`/`-1` for grad >= 0. Match ROOT: only grad < 0 gets `+gstep`. If this breaks the start-at-limit regression (tests/limit_boundary.rs), STOP and report — do not weaken the test or silently re-extend the waiver.
- [ ] F2 Post-escape covariance: ROOT rebuilds diag as SIGNED `1/G2` when `|G2| > prec.Eps()` (can be negative) and marks MnNotPosDef when EDM < 0 (NegativeG2LineSearch.cxx:125-140); Rust V0 build (seed.rs L59-66, L112-119) falls back to 1.0 for every non-positive g2. Check what ROOT MnSeedGenerator.cxx does at ITS V0 site vs what NG2LS does at its rebuild — match each site to its own ROOT form. If matching requires API changes beyond src/migrad/seed.rs (e.g. a not-posdef flag through MinimumError plumbing in other modules), do NOT implement — document it as an explicit remaining-gap row in the audit table with rationale, and say so in the report.
- [ ] F3 Regression test must fail on PRE-change code: the current test calls the new `escape_negative_curvature_with` (wouldn't compile pre-change). Rewrite it to drive the public/pre-existing path (`escape_negative_curvature` or `MigradSeedGenerator::generate`) so the test file alone, applied to the pre-change tree, compiles and FAILS. Prove it: `git stash push src/migrad/seed.rs` is not possible per-hunk — instead, state the exact mechanism you used to demonstrate the pre-change failure and paste its real output into the audit doc (replace the current synthetic "pre-change equivalent failure" block).
- [ ] Audit table rows updated to reflect F1-F3 outcomes (no row claims "implemented" while a known observable gap remains undocumented).
- [ ] Gates: `bash .sc/minuit2-rs-k5h.gate.sh` green; `cargo test --all-features` 0 failures; `python3 scripts/compare_ref_vs_rust.py` stays pass=12 warn=0 fail=0; `python3 scripts/check_executed_surface_gate.py --mode non-regression` PASS. Skip coverage tooling entirely (known env gap — do not retry cargo-llvm-cov, do not use clang++ env hacks).

## 3. Verification
`bash .sc/minuit2-rs-k5h.gate.sh && cargo test --all-features 2>&1 | tail -5 && python3 scripts/compare_ref_vs_rust.py 2>&1 | tail -3 && python3 scripts/check_executed_surface_gate.py --mode non-regression 2>&1 | tail -3`

## 4. Scope
✅ ALWAYS: src/migrad/seed.rs, reports/parity/negative_g2_audit.md, tests/negative_g2.rs (new, if you move the test there), verification/traceability/waiver_rules.csv
⚠️ ASK FIRST: any other src/ module (incl. MinimumError plumbing — see F2), tests/limit_boundary.rs assertions
🚫 NEVER: scripts/run_full_verification.sh coverage phases; global diff thresholds

## 5. Context Pointers
- `br show minuit2-rs-k5h`; VISION.md; third_party/root_ref/math/minuit2/src/NegativeG2LineSearch.cxx and MnSeedGenerator.cxx (pinned, never master).
- The review findings above were independently verified by the supervisor against the pinned source — treat the ROOT line numbers as ground truth to re-check, not as instructions to follow blindly.
- Skills: rust.

## 6. Stop Conditions
- DONE when all criteria pass.
- STOP and report if: F1 breaks limit_boundary; F2 needs out-of-scope API changes (document-and-report path is acceptable); any gate fails twice on the same cause.
