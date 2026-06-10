# Goal: Close the Rosenbrock Migrad NFCN gap (+42% vs C++)   (bead: minuit2-rs-9ma)

## 1. Objective
rosenbrock2_migrad: Rust spends 199 FCN calls where ROOT v6-36-08 spends 140.
Diagnose with the iteration-trace diff tooling, find the mechanism, fix it.
Numerics are already at parity — only the convergence path differs.

## 2. Diagnosis head start (from the mhl cycle — verified)
Divergence starts at ITERATION 0, i.e. the SEED phase: Rust seed nfcn 19 vs
ROOT 9 on rosenbrock2. Start at src/migrad/seed.rs (and what it calls), NOT
the builder loop. Traces already exist under reports/verification/raw/trace/
(rosenbrock2_migrad.{root,rust}.jsonl); regenerate via feature `trace` +
MINUIT2_RS_TRACE_JSONL and diff with scripts/diff_iteration_traces.py.

## 3. Acceptance Criteria
- [ ] rosenbrock2_migrad NFCN rel diff vs ROOT ≤0.10 (ideally exact), with
      fval/param/cov still passing in the differential harness.
- [ ] Mechanism-based fix citing the ROOT source line(s) being mirrored —
      no constant-tuning to luck into 140. If a residual delta is an
      intentional non-replication, document the exact ROOT line + rationale.
- [ ] No other workload regresses in diff_results.csv (pass stays pass;
      the two fixx warns belong to bead b9e — don't touch them).
- [ ] cargo test --all-features green incl. tests/nist_strd_certified.rs;
      a regression test pins the new rosenbrock2 NFCN.
- [ ] README "Pure Rust vs C++ Minuit2" NFCN table regenerated/updated.

## 4. Verification
- Quick: export PATH="$HOME/.cargo/bin:$PATH" && cargo test --test root_reference_minuit2 --test migrad --test nist_strd_certified 2>&1 | tail -20
- Full (logged): scripts/run_full_verification.sh v6-36-08 2>&1 | tail -30
  then inspect rosenbrock rows + status counts in diff_results.csv.

## 5. Scope
✅ ALWAYS: src/migrad/seed.rs, src/migrad/builder.rs, src/linesearch.rs,
   src/gradient/{initial,numerical}.rs, src/parabola.rs, tests/ (new
   regression test), reports/verification/* (regenerated), README.md
   (NFCN table only).
⚠️ ASK FIRST: changes to src/hesse/, src/simplex/, src/minimum/, the trace
   tooling itself, or any tolerance in existing tests.
🚫 NEVER: CHANGELOG.md, .github/, python/, scripts/ (use them, don't edit).

## 6. Non-Goals / Constraints
- The full seed-phase audit (escape_negative_curvature vs ROOT
  NegativeG2LineSearch) is bead k5h — fix what closes THIS gap; if the root
  cause turns out to be exactly that audit's subject, fix the rosenbrock
  mechanism and note the rest for k5h, don't expand scope.
- fixx warnings = bead b9e. Bit-identical 0.5.1 guarantee does NOT bind here;
  fval/param/cov tolerance parity does.

## 7. Context Pointers
- br show minuit2-rs-9ma (full background); VISION.md (item 2: ≤ C++ NFCN).
- ROOT source mirror cited in existing file headers (v6-36-08).
- C++ gotchas list: ~/.claude/.../MEMORY.md mirrored in module rustdoc.

## 8. Stop Conditions
- DONE when all Acceptance Criteria pass.
- STOP and report BLOCKED if: the mechanism requires an ⚠️ Ask-first file,
  the gap cannot drop below 0.10 without constant-tuning, or any previously
  passing workload regresses and the cause is not quickly attributable.
