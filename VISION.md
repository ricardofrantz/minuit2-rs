# VISION

`minuit2-rs` aims to be a **credible replacement for ROOT Minuit2 and iminuit**:
the library a physicist or data scientist can adopt instead of the C++/Python
originals with zero regret.

Concretely, that means:

1. **Same numerics.** Algorithmically equivalent to ROOT Minuit2, validated by
   differential testing against a pinned ROOT release. Divergences are either
   fixed or documented with a reason.
2. **Solves the hard problems the original solves.** Ill-conditioned fits,
   bounded parameters at their limits, fixed-parameter workloads, NIST StRD
   reference problems — a user migrating a difficult fit must not lose
   convergence or pay materially more function evaluations than C++ Minuit2.
3. **Drop-in Python API.** `from minuit2 import Minuit` runs unmodified
   iminuit user code, verified by a differential harness, not by claim.
4. **Evidence over assertion.** Every parity or robustness claim is backed by
   a reproducible gate (ROOT differential workloads, traceability matrix,
   executed-surface mapping, NIST certified oracles, iminuit harness). The
   README states what is and is not yet covered.
5. **Pure Rust, no regrets of its own.** No `unsafe`, no C/C++ toolchain,
   `cargo add` and it builds — the adoption advantages must never be paid for
   with silent numerical drift.

Improvement priority is measured as movement toward this: closing the gaps a
migrating user would actually hit (unconverged hard fits, extra NFCN, missing
iminuit APIs) outranks generic polish.
