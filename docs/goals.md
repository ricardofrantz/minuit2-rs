# Goal: Wire Python Minuit.scan() to the SCAn minimizer   (bead: minuit2-rs-uf5)

## 1. Objective
Minuit.scan(ncall=None) in the PyO3 binding currently raises
NotImplementedError (0.5.0 decision). The Rust core now has the real
brute-force SCAn minimizer (MnScanMinimizer, bead eoi). Implement scan()
matching iminuit semantics and prove it with the differential harness
against real iminuit.

## 2. Acceptance Criteria
- [ ] Minuit.scan(ncall=None) implemented: returns self for chaining, sets
      values + fmin, respects fixed params and limits, FMin flags mirror
      iminuit (validity tied to EDM criterion; covariance semantics — scan
      produces none).
- [ ] ncall mapping replicates iminuit's npoints-per-dimension rule — read
      iminuit's source for the exact rule and cite it in a comment next to
      the harness check.
- [ ] python/compat/diff_iminuit.py extended with ≥2 scan checks (simple
      quadratic from a bad start; bounded parameter case); ALL checks pass
      (29+ total).
- [ ] NotImplementedError tests in python/tests/ replaced by behavior tests
      (on failure, log both libraries' values).
- [ ] README Python Bindings: scan removed from the deferred list.

## 3. Verification
- Env (venv EXISTS from the aic cycle; recreate only if missing):
  . .venv-maturin/bin/activate &&
  PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
- Quick: pytest python/tests/ -q -k scan
- Full (logged): python python/compat/diff_iminuit.py && pytest python/tests/ -q
  && cargo test --all-features (export PATH="$HOME/.cargo/bin:$PATH")

## 4. Scope
✅ ALWAYS: the PyO3 binding source (locate via rg "NotImplementedError" in
   src/), python/minuit2/, python/tests/, python/compat/diff_iminuit.py,
   README.md (Python Bindings section only).
⚠️ ASK FIRST: src/scan/ (the Rust SCAn core — extend only if the binding
   genuinely cannot express iminuit semantics without it), new pip packages.
🚫 NEVER: CHANGELOG.md, .github/, verification/, reports/,
   src/{migrad,hesse,minos,gradient}/.

## 5. Non-Goals / Constraints
- No Rust-core algorithm changes; the SCAn minimizer is done (tests in
  tests/scan_minimizer.rs pin it).
- Don't touch the existing MnParameterScan-based plotting/scan utilities.

## 6. Context Pointers
- br show minuit2-rs-uf5 (incl. venv comment); VISION.md (item 3: drop-in).
- Rust API: src/scan/minimizer.rs rustdoc (when-to-use + no-covariance note).
- Harness style: python/compat/diff_iminuit.py existing 27 checks.

## 7. Stop Conditions
- DONE when all Acceptance Criteria pass.
- STOP and report BLOCKED if: iminuit's actual scan() behavior contradicts
  the bead's description (state what iminuit really does), the venv cannot
  build the extension, or matching FMin validity semantics requires ⚠️ Rust
  core changes.
