# Goal: Per-iteration NFCN trace diff tooling (ROOT vs Rust)   (bead: minuit2-rs-mhl)

## 1. Objective
Build a per-iteration trace channel for Migrad on both the ROOT reference side
and the Rust side, plus a diff script that pinpoints the FIRST iteration where
the two convergence paths split. This unblocks the Rosenbrock (+42% NFCN) and
quadratic3_fixx divergence fixes (VISION: no extra function-evaluation cost vs
C++; divergences fixed or explained with evidence).

## 2. Acceptance Criteria
- [ ] Rust Migrad can emit per-iteration records (iter index, cumulative nfcn,
      fval, edm, line-search lambda, |grad|, dcovar) WITHOUT changing numerics
      or the public API of release builds (test-only or feature-gated hook).
- [ ] ROOT reference runner emits comparable per-iteration records for the
      same workloads (MnTraceObject wiring or MnPrint level-3 stdout parsing —
      either; document the choice in the script header).
- [ ] `scripts/diff_iteration_traces.py --workload rosenbrock2_migrad` prints
      the first divergent iteration (and same for quadratic3_fixx_migrad).
- [ ] Trace JSONL files land under `reports/verification/raw/trace/`.
- [ ] `cargo test --all-features` passes; existing differential outputs are
      byte-identical (numerics untouched).

## 3. Verification
- Quick: export PATH="$HOME/.cargo/bin:$PATH" && cargo test --test root_reference_minuit2 2>&1 | tail -5
- Quick: python3 scripts/diff_iteration_traces.py --workload rosenbrock2_migrad 2>&1 | tail -15
- Full (logged): export PATH="$HOME/.cargo/bin:$PATH" && cargo test --all-features 2>&1 | tail -10

## 4. Scope
✅ ALWAYS: src/migrad/builder.rs (trace hook only), src/migrad/mod.rs,
   scripts/diff_iteration_traces.py (new), verification/ reference-runner
   sources, reports/verification/raw/trace/ (generated), Cargo.toml (only if a
   `trace` feature flag is the chosen mechanism).
⚠️ ASK FIRST: any change that alters NFCN/fval on any workload; any new
   dependency; changes to scripts/run_full_verification.sh stages.
🚫 NEVER: tests/* assertions, README.md, CHANGELOG.md, .github/, src/ files
   other than the migrad trace hook path.

## 5. Non-Goals / Constraints
- NOT this cycle: fixing the divergences themselves (separate beads 9ma/b9e).
- Numerics are sacrosanct: the hook observes, never alters control flow.
- If ROOT-side tracing needs a rebuilt reference runner, reuse the existing
  build flow from scripts/run_full_verification.sh stage 1 — don't fork it.

## 6. Context Pointers
- `br show minuit2-rs-mhl` — full background/reasoning; `VISION.md`.
- Existing end-state harness: scripts/compare_ref_vs_rust.py,
  reports/verification/diff_results.csv, raw JSON in reports/verification/raw/.
- Rust loop: src/migrad/builder.rs; line search: src/linesearch.rs.
- Skills: rust.

## 7. Task Breakdown
1. Rust trace hook + unit smoke (records appear, numerics byte-identical).
2. ROOT-side trace capture for rosenbrock2_migrad + quadratic3_fixx_migrad.
3. diff_iteration_traces.py: align + report first divergence; run on both.

## 8. Stop Conditions
- DONE when all Acceptance Criteria pass.
- STOP and report if: a gate fails twice on the same cause, the ROOT runner
  can't be rebuilt locally, or tracing can't avoid touching numerics.
