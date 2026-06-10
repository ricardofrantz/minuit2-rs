# Goal: 7qf fix round — honest starts, tier tolerances, one grid (bead: minuit2-rs-7qf)

## 1. Objective
Your 7qf work is in the tree (do NOT revert). Review found the hard-tier oracle is
weakened: the multistart anchors are rounded CERTIFIED values, so the test proves
nothing about finding the solution. Fix the three findings; original ACs (goal
commit 74ae336) still apply.

## 2. Acceptance Criteria (this round)
- [ ] F1 HONEST STARTS: no anchor/grid point may be derived from certified values
      (tests/nist_strd_certified.rs ~L579 and examples/nist_strd_hard.rs grids).
      Allowed seeds: the NIST Start 1 / Start 2 vectors from the committed .dat
      files, and deterministic transformations of THOSE (multiplicative grids,
      sign variants, coarse log-space grids, Simplex/SCAn pre-pass results).
      The recipe must genuinely travel from NIST starts to certified values.
      If a dataset cannot reach certified this way within the runtime budget,
      STOP and report with the best worst_rel achieved per start — do not
      smuggle the answer into the seed.
- [ ] F2 TIER TOLERANCES: use the same per-difficulty tolerance scheme as the
      existing 8 plain datasets (check the existing tier mapping in this test
      file; NIST marks Lanczos3 Lower and Hahn1 Average → 1e-3 if that is what
      the existing scheme assigns). No hard-coded 1e-2 acceptance in hard_fit
      when the tier says tighter.
- [ ] F3 ONE GRID: the example and the test must use the IDENTICAL recipe
      (starts, pre-pass, rescaling). Share or duplicate-with-sync-test —
      certification in CI must be achieved by exactly the documented recipe.
- [ ] Determinism double-run still holds; gates: `bash .sc/minuit2-rs-7qf.gate.sh`
      green (clippy is --all-targets — examples are linted; no {:?} prints);
      `cargo test --all-features` 0 failures; default wall-time not regressed.

## 3. Verification
`bash .sc/minuit2-rs-7qf.gate.sh` (plain tier, hard tier ×2, clippy --all-targets)

## 4. Scope
✅ ALWAYS: tests/nist_strd_certified.rs, examples/nist_strd_hard.rs, README.md (Testing)
⚠️ ASK FIRST: any src/ module; .github/workflows/ci.yml
🚫 NEVER: core minimizer numerics; loosening any existing plain-tier tolerance

## 5. Context Pointers
- `br show minuit2-rs-7qf`; reports/parity/nist_hard_baseline.md; the NIST .dat
  files carry Start 1 / Start 2 (e.g. Lanczos3 Start 2 = [0.5, 0.7, 3.6, 4.2, 4, 6.3]).
- Hahn1 rescaling (z = x/1000) stays — but its grid must seed from the NIST
  starts mapped into q-space, not from certified q-values.
- Skills: rust.

## 6. Stop Conditions
- DONE when all criteria pass.
- STOP and report if: any dataset cannot reach certified from honest starts
  (report best per-start worst_rel — that is a real finding, not a failure);
  a gate fails twice on the same cause.
