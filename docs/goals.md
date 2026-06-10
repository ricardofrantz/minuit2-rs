# Goal: Port ROOT ScanBuilder/ScanMinimizer (brute-force SCAn)   (bead: minuit2-rs-eoi)

## 1. Objective
Add ROOT Minuit2's brute-force SCAn as a real minimizer in Rust (currently
src/scan/ only has post-fit profile tools). VISION: robustness tool for bad
starting points + foundation for the iminuit `Minuit.scan()` drop-in (wired in
a separate later bead — NOT this one).

## 2. Acceptance Criteria
- [ ] New scan minimizer in src/scan/ following the existing module pattern
      (like src/simplex/: seed/builder/minimizer), behavior ported from ROOT
      v6-36-08 ScanBuilder.cxx / ScanMinimizer.h (cite the C++ source in
      comments like the rest of the codebase does).
- [ ] Public builder API consistent with MnSimplex/MnMigrad: .add /
      .add_limited / .with_strategy / .max_fcn / .minimize, exported from
      src/lib.rs. Returns FunctionMinimum WITHOUT covariance (like Simplex);
      validity semantics match ROOT's ScanBuilder.
- [ ] New integration test file tests/scan_minimizer.rs: convergence on the
      shared test functions (tests/common.rs quadratic; a bounded-param case;
      a bad-start case showing improvement), detailed assertion messages.
- [ ] Rustdoc on the public type stating when to use it (bad starts, pre-pass
      before Migrad) and that Hesse must follow for errors.
- [ ] cargo test --all-features green; clippy clean; existing differential
      outputs untouched.

## 3. Verification
- Quick: export PATH="$HOME/.cargo/bin:$PATH" && cargo test --test scan_minimizer 2>&1 | tail -10
- Quick: export PATH="$HOME/.cargo/bin:$PATH" && cargo clippy --all-features 2>&1 | tail -5
- Full (logged): export PATH="$HOME/.cargo/bin:$PATH" && cargo test --all-features 2>&1 | tail -10

## 4. Scope
✅ ALWAYS: src/scan/ (new modules), src/lib.rs (export only),
   tests/scan_minimizer.rs (new).
⚠️ ASK FIRST: any change to existing minimizers or MnScan profile behavior;
   any new dependency; adding a differential workload to tools/ref_runner_cpp.
🚫 NEVER: README.md, CHANGELOG.md, python bindings, .github/, existing tests,
   reports/.

## 5. Non-Goals / Constraints
- NOT this cycle: Python Minuit.scan() wiring (bead minuit2-rs-uf5); README
  docs (supervisor handles docs after the Python bead).
- Keep the profile-tool MnScan and the new minimizer clearly distinct in
  naming and rustdoc (0.5.0 had naming-confusion pain here).
- Port behavior with the pinned ROOT source open: third_party checkout used by
  scripts/build_root_reference_runner.sh, tag v6-36-08 — not master.

## 6. Context Pointers
- `br show minuit2-rs-eoi` — full background/reasoning; `VISION.md`.
- Existing patterns: src/simplex/{seed,builder,minimizer}.rs, src/scan/mod.rs,
  src/minimum/ (FunctionMinimum construction), tests/simplex.rs (test style).
- ROOT reference: ScanBuilder.cxx, ScanMinimizer.h, MnScan.h at v6-36-08.
- Skills: rust.

## 7. Task Breakdown
1. Read ScanBuilder.cxx at the pinned tag; map its loop to a Rust builder.
2. Implement seed/builder/minimizer + public API; unit smoke.
3. tests/scan_minimizer.rs (quadratic, bounded, bad-start) → quick gate.
4. clippy + full test suite → full gate.

## 8. Stop Conditions
- DONE when all Acceptance Criteria pass.
- STOP and report if: a gate fails twice on the same cause, the pinned ROOT
  source is unavailable locally, or matching ROOT validity semantics requires
  touching shared minimum/state types (⚠️ ask first).
