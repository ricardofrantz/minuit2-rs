# Goal: Implement Python Minuit.scan() — iminuit hypercube semantics   (bead: minuit2-rs-uf5, RE-BRIEF)

## 1. Objective (corrected mechanism)
Previous brief said "wire to MnScanMinimizer" — WRONG, verified against
iminuit 2.32.0 source: real iminuit.Minuit.scan() is a full-hypercube grid
scan implemented in Python, and its docstring explicitly deems Minuit2's
MnScan unsuitable (sequential 1D 41-step scans fail on correlated params).
So: implement Minuit.scan(ncall=None) by REPLICATING iminuit's hypercube
algorithm. Do NOT wire MnScanMinimizer to Python scan(); it stays as
ROOT-parity Rust surface (tests/scan_minimizer.rs untouched).

## 2. iminuit semantics to replicate (from inspect.getsource, v2.32.0)
- nstep = int(ncall ** (1/nfit)); ncall default = the migrad maxcall
  heuristic. Cite this rule in a comment next to the harness check.
- Grid bounds per param: limits if set, else value ± error; fixed params
  pinned at value; function IS evaluated at the boundary (linspace incl.
  endpoints).
- Best grid point → values; EDM computed only AFTER the scan at the best
  point; validity = EDM criterion (invalid scan result is normal);
  covariance reset to None; fmin algorithm label "Scan"; returns self.

## 3. Acceptance Criteria
- [ ] Minuit.scan(ncall=None) implemented with the semantics above; the
      NotImplementedError is gone.
- [ ] python/compat/diff_iminuit.py: ≥2 new scan checks vs real iminuit
      (simple quadratic from a bad start; bounded parameter case); ALL
      checks pass (29+ total).
- [ ] python/tests/: NotImplementedError tests replaced by behavior tests
      (on failure, log both libraries' values).
- [ ] README Python Bindings: scan removed from the deferred list.
- [ ] cargo test --all-features still green; tests/scan_minimizer.rs
      untouched.

## 4. Verification
- Env (venv exists): . .venv-maturin/bin/activate &&
  PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
- Quick: pytest python/tests/ -q -k scan
- Full (logged): python python/compat/diff_iminuit.py && pytest python/tests/ -q
  && (export PATH="$HOME/.cargo/bin:$PATH" && cargo test --all-features)

## 5. Scope
✅ ALWAYS: PyO3 binding source (rg "NotImplementedError" in src/),
   python/minuit2/, python/tests/, python/compat/diff_iminuit.py,
   README.md (Python Bindings section only).
⚠️ ASK FIRST: src/scan/, src/minimum/ (only if constructing a valid
   fmin/EDM for the scanned point genuinely needs core support — explain
   what and why), new pip packages.
🚫 NEVER: CHANGELOG.md, .github/, verification/, reports/,
   src/{migrad,hesse,minos,gradient}/, tests/scan_minimizer.rs.

## 6. Context Pointers
- br show minuit2-rs-uf5 — incl. the course-correction comment;
  VISION.md item 3 (drop-in).
- Exact iminuit reference: .venv-maturin/.../iminuit/__init__.py
  Minuit.scan (read it via inspect.getsource before coding).

## 7. Stop Conditions
- DONE when all Acceptance Criteria pass.
- STOP and report BLOCKED if: EDM/validity semantics cannot be matched
  without ⚠️ core changes, or a differential scan check disagrees with
  iminuit beyond grid-tie noise (report both values).
