# Improvement Ledger

Decisions about improvement ideas live here (accepted / rejected / deferred).
Work items live in the bead database. See `VISION.md` for the north star.

## Run 2026-06-10 — initial

**Studied:** full first-run study. v0.5.1: 148 tests, ROOT differential
10 pass / 2 warn / 0 fail, iminuit harness 20/20, traceability unresolved=0,
executed-surface P1=48. Known weaknesses: 4 NIST StRD datasets unsolved from
Start 2 (Lanczos3, BoxBOD, MGH09, Hahn1); Rosenbrock Migrad +42% NFCN vs C++;
quadratic3_fixx migrad/hesse NFCN warnings.

**Direction (user-confirmed):** north star = credible iminuit/Minuit2
replacement (VISION.md written this run). Focus axis = **hard-problem
robustness** only.

**Ranked ideas (impact vs effort, toward VISION):**

1. **Close the Rosenbrock NFCN gap (+42% vs C++)** — diff the per-iteration
   ROOT trace against the Rust path (line search, gradient step sizing,
   MnParabola edge cases) to find where convergence paths split; fix the
   divergence. Impact H (headline "what you give up" item), effort M.
2. **Fix the two quadratic3_fixx NFCN warnings (migrad +42%, hesse +23%)** —
   fixed-parameter seed/Hesse path divergence; turning warn=2 into warn=0
   makes the differential gate fully green. Impact M-H, effort M.
3. **Brute-force SCAn minimizer** — port ROOT's ScanBuilder/ScanMinimizer as a
   real minimizer (currently only parameter scans exist); wires up iminuit's
   deferred `Minuit.scan()` and gives users a robustness tool for bad starts.
   Impact M-H (robustness + removes a NotImplementedError), effort M.
4. **Multistart recipe for the 4 unsolved NIST datasets** — first establish
   what iminuit/ROOT do from Start 2 on Lanczos3/BoxBOD/MGH09/Hahn1 (parity
   baseline, not magic); then provide a documented, tested multistart/rescaling
   recipe (example-level or small helper) that reaches the certified values,
   and promote those datasets from "skipped" to "solved via recipe" in tests.
   Impact H toward "solves the hard problems", effort M.
5. **Seed-phase parity audit (NegativeG2LineSearch)** — v0.5.0 added
   `escape_negative_curvature` for the g2<=0 seed case; audit it against
   ROOT's full NegativeG2LineSearch (which also runs during iteration, not
   only at seed) and close any behavioral gap. Impact M, effort S-M.

**Recommended cut line: after idea 5 (all five).**

**Rejected:**
- Internal auto-rescaling of ill-conditioned parameters — ROOT doesn't do it;
  silent rescaling risks numerical divergence from the reference, violating
  VISION item 1. Recipe-level rescaling (idea 4) covers the need.
- Weakening NIST oracle tests to "close enough" tolerances — violates
  evidence-over-assertion.

**Deferred (out of chosen axis this run):**
- iminuit drop-in completion beyond `scan()` (mncontour cl=, grad/hessian
  callbacks, iminuit.cost) — drop-in axis, not selected.
- Executed-surface P1=48 burn-down to strict-gate green — verification axis.
- Fair wall-clock benchmark vs C++ — verification axis.
- Rust ergonomics / autodiff (num-dual) / Fumili / WASM — ergonomics axis.

**Accepted → beads** (cut line set by user: all five; `.beads/` gitignored by
user choice — bead DB is local-only, this ledger is the committed record):
- Idea 1 (Rosenbrock NFCN gap) — `minuit2-rs-mhl` (trace-diff tooling, P0) +
  `minuit2-rs-9ma` (the fix, P1, dep: mhl)
- Idea 2 (fixx warnings) — `minuit2-rs-b9e` (P1, dep: mhl); audit reframed it:
  Rust uses FEWER calls (rel diff 0.828/0.513), so the bead checks for
  under-iteration, not slowness
- Idea 3 (SCAn minimizer) — `minuit2-rs-eoi` (Rust core, P2) +
  `minuit2-rs-uf5` (Python `Minuit.scan()`, P2, dep: eoi)
- Idea 4 (NIST hard datasets) — `minuit2-rs-aic` (upstream baseline first, P2)
  + `minuit2-rs-7qf` (recipe + tests, P2, dep: aic)
- Idea 5 (seed parity audit) — `minuit2-rs-k5h` (P2, dep: 9ma — same loop,
  avoid churn)

**Phase 4 audit:** no dep cycles; 3 parallel ready tracks (mhl, eoi, aic);
critical path mhl→9ma→k5h; quick tier verified (`cargo test --test
root_reference_minuit2` 5/5 pass); `.venv-maturin` missing in this checkout —
setup note added as comments on uf5/aic. Reproducibility: multistart bead
mandates deterministic seeding; ROOT ref commit and NIST data already pinned.
