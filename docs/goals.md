# Goal: Seed-phase parity audit — escape_negative_curvature vs ROOT NegativeG2LineSearch (bead: minuit2-rs-k5h)

## 1. Objective
Audit every call site and the internal algorithm of ROOT's NegativeG2LineSearch
(pinned v6-36-08, commit a8ca1b23e38d7dbe0ff24027894ca0f2ad65f1bd) against our
`escape_negative_curvature` in src/migrad/seed.rs and the Migrad builder loop;
implement genuine gaps (e.g. mid-iteration negative-g2 handling) with regression
tests, or document intentional differences with waiver rationale. VISION item 1
(same numerics, divergences fixed or documented).

## 2. Acceptance Criteria
- [ ] Findings table at reports/parity/negative_g2_audit.md: call site | ROOT behavior | Rust behavior | gap? — covering MnSeedGenerator AND any in-iteration use (VariableMetricBuilder, MnApplication/minimize strategy retries) in v6-36-08.
- [ ] Every genuine gap implemented has a regression test that FAILS on the pre-change code (state the pre-change failure output in the report).
- [ ] Intentional gaps: rationale in the audit doc + verification/traceability/waiver_rules.csv if applicable.
- [ ] Quick gate green; `cargo test --all-features` 0 failures; clippy clean.
- [ ] If numerics changed: regenerate differential baselines via scripts/run_full_verification.sh v6-36-08; diff_summary must stay pass≥10 fail=0 (waived warns from b9e stay waived). If NO numerics changed, run it anyway and confirm diff_results.csv unmodified.

## 3. Verification
Quick gate (both sides run): `bash .sc/minuit2-rs-k5h.gate.sh`
Full (coder runs once, logs output in report): `scripts/run_full_verification.sh v6-36-08`

## 4. Scope
✅ ALWAYS: src/migrad/seed.rs, src/migrad/builder.rs (only if an in-iteration gap is confirmed), tests/negative_g2.rs (new) or tests/limit_boundary.rs, reports/parity/negative_g2_audit.md, verification/traceability/waiver_rules.csv
⚠️ ASK FIRST: changes to any other src/ module; changing global diff thresholds or workload waivers
🚫 NEVER: verification/workloads/*.json (except via the full script's regeneration), README NFCN table claims from 9ma

## 5. Non-Goals / Constraints
- NOT this cycle: NIST multistart recipe (bead 7qf); performance work.
- Audit against the PINNED checkout used by scripts/run_full_verification.sh (find its reference-source path inside the script) — never master. If the checkout lacks the two files, fetch NegativeG2LineSearch.{h,cxx} at the pinned commit and save under reports/verification/raw/ for citation.
- Cite ROOT file:line for every row of the findings table.

## 6. Context Pointers
- `br show minuit2-rs-k5h` — full background (the 0.5.0 start-at-limit fix, INVENTORY.md status) and reasoning.
- VISION.md; src/migrad/seed.rs (escape_negative_curvature), src/migrad/builder.rs (main loop), tests/limit_boundary.rs (test style to extend).
- Prior ledger context: divergence fixed in 9ma was the gradient calculator, NOT escape_negative_curvature — the in-iteration question is still open.
- Skills: rust.

## 7. Task Breakdown
1. Locate pinned ROOT source; read NegativeG2LineSearch.{h,cxx} + grep all call sites → draft findings table.
2. Diff each call site/algorithm step vs Rust → classify gap / parity / intentional.
3. For each genuine gap: write the failing regression test first, then implement, cite ROOT lines in code comments.
4. Run quick gate, full suite, full verification script → finalize report.

## 8. Stop Conditions
- DONE when all Acceptance Criteria pass.
- STOP and report if: a gate fails twice on the same cause; an ⚠️ ASK-FIRST file needs touching; the pinned ROOT source cannot be located/fetched.
