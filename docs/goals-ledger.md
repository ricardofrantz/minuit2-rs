# Supervisor-coder ledger

## 2026-06-10 — minuit2-rs-mhl: Per-iteration NFCN trace diff tooling
- AC1 Rust trace hook (feature `trace`, env MINUIT2_RS_TRACE_JSONL, no-op when off): PASS (diff reviewed; numerics untouched)
- AC2 ROOT-side records (MnTraceObject → JSONL; lambda=null limitation documented): PASS
- AC3 diff_iteration_traces.py first-divergence report: PASS (re-ran: rosenbrock2 iter 0, fixx iter 0)
- AC4 traces under reports/verification/raw/trace/: PASS
- AC5 cargo test all-features + byte-identical differential outputs: PASS (re-ran quick tier; diff_results.csv unmodified, pass=10 warn=2 fail=0)
- Finding for 9ma: divergence starts at iteration 0 — seed phase (Rust nfcn 19 vs ROOT 9 on rosenbrock; fixx Rust nfcn 1 vs ROOT 7). Start diagnosis at src/migrad/seed.rs, not the loop.
- Follow-ups: none filed (trace alignment semantics may need refining during 9ma; noted in bead 9ma context via this ledger).

## 2026-06-10 — minuit2-rs-eoi: Port ScanBuilder/ScanMinimizer (brute-force SCAn)
- AC1 src/scan/{seed,builder,minimizer}.rs, ROOT v6-36-08 cited in headers: PASS
- AC2 MnScanMinimizer builder API (.add/.add_limited/.with_strategy/.max_fcn/.minimize), exported, no covariance: PASS (diff reviewed)
- AC3 tests/scan_minimizer.rs (quadratic, bounded, bad-start) with detailed messages: PASS (re-ran: 3/3)
- AC4 rustdoc when-to-use + run-Hesse-after note: PASS
- AC5 cargo test --all-features green (re-ran: 0 failures), clippy clean (re-ran), differential outputs untouched: PASS
- Note: line-by-line fidelity to ScanBuilder.cxx not differentially verified (optional path not taken); structure matches ROOT (SimplexSeedGenerator composition, 41-point grid, ±2σ clamped to limits, sequential updates). Bead uf5's iminuit harness scan checks will validate behavior against the C++-backed iminuit.
- Follow-ups: none.

## 2026-06-10 — minuit2-rs-aic: NIST hard-dataset baseline
- AC1 baseline script, 4 datasets x {iminuit,minuit2-rs} x {s1,s2}: PASS (re-ran myself, deterministic, 0.17 s)
- AC2 shared python/compat/nist_models.py parsing committed .dat files: PASS (verified import + parser)
- AC3 reports/parity/nist_hard_baseline.md matrix + per-dataset conclusions: PASS
- AC4 genuine gap flagged at top: PASS — Hahn1 (iminuit s1 reaches certified, fval=1.53244; minuit2-rs valid=False both strategies)
- AC5 deterministic, <5 min: PASS
- Findings: Lanczos3 + MGH09 = parity failures (iminuit also fails from Start 2) → recipe targets. Hahn1 = GENUINE GAP → core investigation, likely seed/conditioning (relates to bead k5h). BoxBOD passes BOTH libs at 1e-2 from Start 2 — the skip note in tests/nist_strd_certified.rs appears stale; recipe bead should verify and possibly promote it to a plain oracle test.
- Follow-ups: comments added to bead minuit2-rs-7qf (re-scope: Hahn1 core gap, BoxBOD promotion check).

## 2026-06-10 — minuit2-rs-9ma: Close the Rosenbrock Migrad NFCN gap
- AC1 rosenbrock2_migrad nfcn_rel ≤0.10: PASS — 0.296 → 0.00714 (Rust 139 vs ROOT 140); strategy2 0.186 → 0.0926; fval/param/cov all pass.
- AC2 mechanism-based fix: PASS — numerical.rs now mirrors Numerical2PGradientCalculator.cxx L54-151: epspri=eps2+|grd*eps2| curvature floor, vrysml=8*eps*eps, pre-evaluation step-convergence break, grad/g2/gstep seeded from previous gradient (not zeros). ROOT lines cited in module docs.
- AC3 no status regressions: PASS — all 10 pass stay pass, fixx warns unchanged (b9e). Collateral NFCN shifts within pass: lower_limited 49→33 (now −27% vs ROOT), scan_p0 nfcn_rel 0→0.357.
- AC4 tests green + regression pin: PASS — cargo test --all-features 0 failures (re-ran myself); new root_migrad_rosenbrock2_nfcn_stays_at_root_parity pins nfcn=139; clippy clean (re-ran).
- AC5 README NFCN table: PASS — table + prose updated; "+42%" claim replaced with measured parity.
- Note: coder exited BLOCKED (wrapper exit 4) solely on run_full_verification.sh's coverage step — cargo-llvm-cov not installed on this box (ask-first honored). diff_results.csv + all raw refs regenerated fine; ROOT ref JSONs shifted at ~1e-9 (runner rebuild), nfcn refs unchanged.
- Follow-ups: env — install cargo-llvm-cov (user decision) so the full script's coverage tier runs; bead k5h (seed audit) remains relevant — divergence fixed here was the gradient calculator, not escape_negative_curvature.

## 2026-06-10 — minuit2-rs-b9e: Resolve quadratic3_fixx NFCN warnings (route b: waiver)
- AC1 diff_summary warn=0: PASS — re-ran compare_ref_vs_rust.py myself: pass=12 warn=0 fail=0. Mechanism: per-workload `nfcn_rel_waiver` string in verification/workloads/root_minuit2_v6_36_08.json; "waived" warnings excluded from status; global 0.5 threshold untouched.
- AC2 written explanation citing ROOT source: PASS — known_differences.md D-001 cites MnSeedGenerator.cxx (start-point eval + gc(pa)), Numerical2PGradientCalculator.cxx (central-diff probes per variable param per cycle), InitialGradientCalculator.cxx (heuristic, no FCN). Consistent with the mhl trace (iter-0 fixx: Rust nfcn 1 vs ROOT 7).
- AC3 (route-a only) n/a — correctness already within tolerance (param max abs 3.9e-4 < 5e-4 tol; hesse cov 9.9e-10).
- AC4 tests + no regressions: PASS — cargo test --all-features 0 failures (re-ran); rosenbrock rows keep nfcn_rel 0.0071/0.093; only the 5 in-scope files changed.
- Supervisor reservation: the waiver explains the iteration-0 seed accounting, but the TOTAL gap (5 vs 29 calls) implies ROOT also spends per-iteration gradient probes Rust skips; bead k5h (seed-phase parity audit) owns the deeper question — waiver's next-step clause points back to route (a) if quality ever degrades.
- Follow-ups: none new (k5h already filed).

## 2026-06-10 — minuit2-rs-uf5: Python Minuit.scan() — iminuit hypercube semantics (re-briefed)
- COURSE CORRECTION: original bead said "wire to MnScanMinimizer"; verified iminuit 2.32.0 source — scan() is a pure-Python full-hypercube grid scan and its docstring deems MnScan unsuitable (1D sequential, fails on correlated params). Re-briefed to replicate iminuit's algorithm; MnScanMinimizer stays ROOT-parity surface only (bead comment + goal commit a4d4d65 carry the evidence).
- AC1 scan(ncall=None) semantics: PASS — hypercube recursion, nstep=int(ncall^(1/nfit)), linspace incl. boundaries, limits-else-value±error per side, fixed pinned, covariance→None, EDM-based validity, chaining (migrad/simplex clear scan fmin).
- AC2 ncall rule cited: PASS (comment at the grid code; supervisor touch-up: default heuristic now uses nfit per iminuit _migrad_maxcall, was npar).
- AC3 differential harness: PASS — re-ran: PASS=29 incl. scan_quadratic_bad_start + scan_bounded_parameter vs real iminuit.
- AC4 behavior tests: PASS — pytest 12 passed; NotImplementedError test replaced, failures log both libs' values.
- AC5 README deferred list: PASS — scan removed.
- AC6 Rust suite: PASS — cargo test --all-features 0 failures, clippy clean (re-ran after touch-up).
- Follow-ups: hesse()/minos() after scan() returns None-minimum path (iminuit allows hesse after scan) — minor drop-in edge, not harness-covered; consider in a future binding-parity bead.

## 2026-06-10 — minuit2-rs-k5h: Seed-phase parity audit vs ROOT NegativeG2LineSearch
- AC1 findings table (reports/parity/negative_g2_audit.md, ROOT file:line cited): PASS — key result: ROOT v6-36-08 has NO in-iteration NegativeG2LineSearch call (VariableMetricBuilder uses MnPosDef only), so builder.rs correctly untouched; seed-phase had 3 genuine gaps, all fixed.
- AC2 regression fails pre-change: PASS — supervisor-proved by stashing seed.rs: tests/negative_g2.rs FAILED (0 passed; 1 failed) on pre-change, passes post-change. Drives public MigradSeedGenerator::generate.
- AC3 intentional gaps waived: PASS — start-at-limit waiver extended to cover escape direction (+probe) at the zero-Jacobian singularity ROOT skips (NegativeG2LineSearch.cxx:68-71); analytical-G2 seed gap documented (needs src/gradient/analytical.rs, out of scope).
- AC4 gates: PASS (re-ran all myself) — quick gate exit 0, cargo test --all-features 0 failures, clippy clean, differential pass=12 warn=0 fail=0 (diff_results.csv unchanged), executed-surface gate PASS (P0=0 P1=48 P2=425 = baseline).
- Fixes landed: per-coordinate repair + full-gradient recompute (was all-coords vector step, NegativeG2LineSearch.cxx:62-123); ROOT step direction -gstep for grad>=0 in ROOT-reachable branch (cxx:80-83); signed 1/G2 covariance rebuild + EDM<0 not-posdef flag at the NG2LS site only (cxx:125-140).
- Cycle notes: 2 fix rounds. Round 1 coder BLOCKED on cargo-llvm-cov (known env gap, still open) + clang++ probe false-alarmed executed-surface gate (ABI symbol drift); Codex review found the 3 real gaps. Round 2 stop condition fired correctly on F1 vs limit_boundary; resolved by branch split (ROOT direction where ROOT executes, waived direction where ROOT skips).
- Follow-ups: none new (cargo-llvm-cov install remains a standing user decision from 9ma).

## 2026-06-10 — minuit2-rs-7qf: Multistart/rescaling recipe for the 4 hard NIST datasets
- AC1 BoxBOD promoted to plain tier: PASS — plain tier now 9 passed (stale skip note removed).
- AC2 recipe (examples/nist_strd_hard.rs): PASS — NIST Start 1/2 grids only (parsed from committed .dat), Lanczos3 profiled-LS pre-pass, Simplex→Migrad via MnMinimize, final Migrad+Hesse, Hahn1 z=x/1000 user-model rescaling; certified values used for assertions only.
- AC3 hard test tier: PASS — nist_hard_via_recipe (#[ignore]) certifies Lanczos3 (worst_rel 1.5e-10, tol 1e-3), MGH09 (9.96e-4, tol 1e-2), Hahn1 (2.98e-5, tol 1e-3); per-start logging; tolerances follow existing difficulty-tier scheme.
- AC4 determinism: PASS — double-run identical (gate runs hard tier twice).
- AC5 README Testing: PASS — 9 plain + 3 via documented recipe, cites nist_hard_baseline.md.
- AC6 Hahn1 core bead filed: PASS — minuit2-rs-j2j (source: supervisor cycle minuit2-rs-7qf); k5h did NOT fix the core gap (re-ran baseline: statuses unchanged, NFCN shifts only, committed as chore 611dea5).
- AC7 gates: PASS (re-ran all myself) — gate exit 0, cargo test --all-features 0 failures, clippy --all-targets clean, default wall-time unaffected (hard tier ignored).
- Cycle notes: 1 fix round — initial submission seeded multistart anchors with ROUNDED CERTIFIED VALUES (weakened oracle, caught by Codex review, verified vs .dat starts), used 1e-2 for 1e-3-tier datasets, and had test/example grid drift; all three re-briefed and fixed. Supervisor touch-up: 2 clippy use_debug lints in example (coder ran clippy without --all-targets).
- Follow-ups: minuit2-rs-j2j (Hahn1 core divergence) already filed by the cycle.

## 2026-06-11 — minuit2-rs-j2j: Hahn1 core divergence (diagnose + ROOT-parity fix)
- AC1 diagnosis: PASS — reports/parity/hahn1_core_divergence.md: valid=False was above_max_edm (EDM ~1e16 sanity blowup) after stopping in a local basin with dcovar>0.05; first trajectory divergence at accepted iter 0 (traces committed). Mechanism: missing ROOT Hesse-verified continuation (VariableMetricBuilder.cxx:112-151 — verified against vendored source line-by-line).
- AC2 fix + regression: PASS — Hesse-verified continuation, EDM/update ordering (cxx:291-312), negative-delgam Davidon updates (DavidonErrorUpdator.cxx:42-78). tests/hahn1_core_parity.rs proven failing pre-change by supervisor (stashed builder.rs: FAILED, valid=False fval≈51623). Post-fix: Hahn1 s1 reaches certified fval=1.53244 from honest NIST starts, nfcn=474 (iminuit 581); s2 fails like iminuit s2 (parity).
- AC3 no regressions: PASS (re-ran all myself) — differential harness pass=12 warn=0 fail=0; NIST plain 9 passed; hard tier ×2 passes; baseline regenerated: Hahn1 parity-success, Lanczos3/MGH09 improved (still FAIL like iminuit).
- AC4 gates: PASS — gate exit 0, cargo test --all-features 0 failures, clippy --all-targets clean.
- Cycle notes: 2 fix rounds, both from Codex reviews adjudicated against ROOT source. Round 1 (5 P2s): Hesse runs on strategy condition not only nominal convergence; Hesse state always appended (covariance of result = Hesse-verified, new test); invalid-Hesse exit checks error validity; Hesse budget = original maxfcn; NaN/negative-EDM guard with MnPosDef re-estimate (cxx:301-322). Round 2 (3 P2s, failure paths): 1.3x continuation budget propagates to call-limit status (minimizer.rs reordered, new test); Hesse sag failure returns MnHesseFailed diagonal state (MnHesse.cxx:242-267, new flat-parameter test); invalid Hesse state no longer becomes the returned minimum.
- Scope deviation accepted: src/hesse/calculator.rs rewritten to MnHesse.cxx parity (yy cross-term caching = MnHesse.cxx:198/276/349, gstep-seeded steps, sag retry loop) — beyond "read-only wiring" but verified faithful; covariance shifts ~1e-10; D-001 waiver still covers fixx-hesse nfcn accounting (waived rel 0.513→0.667, mechanism unchanged). rosenbrock2 NFCN pin 139→152 (verification calls now counted; nfcn_rel 0.086 still ≤ 9ma's 0.10 criterion). Remaining churn = cargo fmt reflow.
- Follow-ups: none new (cargo-llvm-cov install remains a standing user decision).
