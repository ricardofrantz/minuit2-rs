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
