# Goal: Multistart/rescaling recipe for the 4 hard NIST datasets (bead: minuit2-rs-7qf)

## 1. Objective
Documented, tested, deterministic recipe (example + test tier + README) that reaches
NIST-certified parameters for Lanczos3, BoxBOD, MGH09, Hahn1 — explicit user-level
strategy, core algorithms untouched (VISION items 2, 4, 5).

## 2. Acceptance Criteria
- [ ] BoxBOD: baseline (reports/parity/nist_hard_baseline.md, just regenerated) shows it passes PLAIN in both libs — verify and promote it to the plain certified-oracle tier in tests/nist_strd_certified.rs; remove the stale skip note.
- [ ] Lanczos3, MGH09, Hahn1: recipe per dataset in an examples/ hard-mode example (extend existing nist example if present): deterministic multistart grid (fixed constant seed/grid, NO time-based RNG), Simplex or SCAn pre-pass, explicit variable rescaling in the USER model where needed (Hahn1), then Migrad+Hesse.
- [ ] Same 3 datasets promoted from skipped to a hard tier in tests/nist_strd_certified.rs (`#[ignore]`d or named tier, e.g. nist_hard_via_recipe) asserting certified values at the same tolerance scheme as the existing 8, with per-start logging on failure.
- [ ] Determinism: two consecutive runs of the hard tier produce identical outcomes (show both runs in the report).
- [ ] README Testing section: which pass plain (now 9 incl. BoxBOD) vs via recipe (3), citing reports/parity/nist_hard_baseline.md (iminuit also fails Lanczos3/MGH09 plain; Hahn1 see next line).
- [ ] File a NEW bead (br create) for the Hahn1 core divergence: iminuit s1 reaches certified (fval=1.53244) but minuit2-rs is valid=False both strategies EVEN AFTER the k5h seed fix (nfcn 260 s2, fval 46949.9) — comment `source: supervisor cycle minuit2-rs-7qf`. The recipe may still cover Hahn1, but do NOT claim the core gap is resolved.
- [ ] cargo test --all-features green; DEFAULT test wall-time not materially regressed (hard tier ≤~60 s or #[ignore] + explicit filter).

## 3. Verification
Quick: `export PATH="$HOME/.cargo/bin:$PATH" && cargo test --test nist_strd_certified 2>&1 | tail -10` (plain tier incl. BoxBOD)
Hard tier: `cargo test --test nist_strd_certified -- --ignored 2>&1 | tail -20` (or the chosen filter) — run TWICE for determinism.
Full: `cargo test --all-features 2>&1 | tail -5`; clippy `-D warnings`.

## 4. Scope
✅ ALWAYS: tests/nist_strd_certified.rs, examples/ (nist hard-mode), README.md (Testing section only), python/compat/nist_models.py (read-only reference)
⚠️ ASK FIRST: any src/ module (a multistart driver utility is allowed ONLY if clearly outside the parity surface, deterministic, documented as an extension over C++ Minuit2 — prefer example-level code; ask before creating it), .github/workflows/ci.yml
🚫 NEVER: core minimizer numerics (src/migrad/, src/simplex/ internals), scripts/nist_hard_baseline.py results massaging, global tolerances

## 5. Non-Goals / Constraints
- NOT this cycle: fixing the Hahn1 core divergence (file the bead instead); auto-rescaling inside the core (REJECTED in ledger).
- Recipe = explicit operator strategy mirroring ROOT-world practice; every start/rescale choice must be stated in the example's comments.

## 6. Context Pointers
- `br show minuit2-rs-7qf` (incl. the baseline comment); VISION.md; reports/parity/nist_hard_baseline.md (fresh numbers, post-k5h).
- tests/nist_strd_certified.rs (existing 8 + skip notes); examples/ for prior example style; MnMinimize (src/minimize/), MnScanMinimizer (src/scan/minimizer.rs) as pre-pass building blocks.
- Skills: rust.

## 7. Task Breakdown
1. Verify BoxBOD plain → promote → quick gate.
2. Build recipe per dataset in the example; iterate until certified values hit.
3. Wire hard test tier + determinism double-run.
4. README + file the Hahn1 bead → full gates.

## 8. Stop Conditions
- DONE when all criteria pass.
- STOP and report if: any of the 3 cannot reach certified under a deterministic recipe within the runtime budget; a src/ helper seems required (ask first); a gate fails twice on the same cause.
