# Goal: k5h fix round 2 — F1 resolved by branch split; finish and verify (bead: minuit2-rs-k5h)

## 1. Objective
Your fix-round work is in the tree (do NOT revert). You stopped correctly on F1:
ROOT's zero-grad direction breaks tests/limit_boundary.rs. Supervisor decision below
resolves it. Finish F1 with this resolution, keep your F2/F3 work, pass all gates.

## 2. The F1 resolution (authorized)
The conflict only exists in the case ROOT never reaches: at a limit, grad≈0 AND
g2≈0 (both < eps) — ROOT's skip (NegativeG2LineSearch.cxx:68-71) `continue`s and
never escapes (the upstream bug our 0.5.0 waiver fixes). ROOT's else-branch
direction (-gstep for grad >= 0) only governs cases ROOT actually executes:
grad == 0 with |g2| > eps. So split:
- ROOT-reachable branch (|g2| > eps, grad >= 0): step = -gstep  ← ROOT parity (F1)
- ROOT-reachable branch (grad < 0): step = +gstep               ← already matches
- WAIVED branch only (|grad| < eps AND |g2| < eps, nonzero gstep — the
  start-at-limit case ROOT skips): keep step = +gstep, our documented choice.
Update the waiver text in reports/parity/negative_g2_audit.md to cover the
direction choice explicitly (ROOT defines no direction here because it skips).
tests/limit_boundary.rs must pass UNCHANGED — its assertions are ground truth.

## 3. Acceptance Criteria
- [ ] F1 implemented as the branch split above; limit_boundary passes unchanged.
- [ ] F2 (signed 1/G2 covariance per site) and F3 (regression test compiles and
      FAILS on pre-change tree, real pre-change output pasted in audit doc) from
      the previous brief remain satisfied — state their status in the report.
- [ ] Audit table + waiver rows consistent with the final code.
- [ ] Gates: `bash .sc/minuit2-rs-k5h.gate.sh` green; `cargo test --all-features`
      0 failures; `python3 scripts/compare_ref_vs_rust.py` pass=12 warn=0 fail=0;
      `python3 scripts/check_executed_surface_gate.py --mode non-regression` PASS.
      No coverage tooling, no clang++ env hacks.

## 4. Scope
✅ ALWAYS: src/migrad/seed.rs, reports/parity/negative_g2_audit.md, tests/negative_g2.rs, verification/traceability/waiver_rules.csv
⚠️ ASK FIRST: any other src/ module; tests/limit_boundary.rs (read-only ground truth)
🚫 NEVER: coverage phases; global diff thresholds

## 5. Context Pointers
- `br show minuit2-rs-k5h`; third_party/root_ref (pinned); previous brief: goal commit 15ac294.
- Skills: rust.

## 6. Stop Conditions
- DONE when all criteria pass.
- STOP and report if: the branch split still cannot satisfy both limit_boundary and
  the ROOT-reachable direction; any gate fails twice on the same cause.
