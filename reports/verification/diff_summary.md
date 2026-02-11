# Differential Verification Summary

Reference repo: `https://github.com/root-project/root`
Reference subtree: `math/minuit2`
Reference tag: `v6-36-08`
Reference commit: `a8ca1b23e38d7dbe0ff24027894ca0f2ad65f1bd`

## Status Counts

- pass: **4**
- warn: **2**
- fail: **0**

## Per-Workload Results

| Workload | Status | Issues | Warnings |
|---|---|---|---|
| `quadratic3_fixx_migrad` | `warn` | - | nfcn relative diff 0.828 > 0.5 |
| `quadratic3_fixx_hesse` | `warn` | - | nfcn relative diff 0.513 > 0.5 |
| `rosenbrock2_migrad` | `pass` | - | - |
| `quadratic2_minos_p0` | `pass` | - | - |
| `quadratic2_simplex` | `pass` | - | - |
| `rosenbrock2_minimize` | `pass` | - | - |

## Artifacts

- `reports/verification/diff_results.csv`
- `reports/verification/raw/ref/*.json`
- `reports/verification/raw/rust/*.json`

## Notes

- `fail` means correctness metrics exceeded workload tolerances.
- `warn` means correctness metrics passed, but NFCN divergence exceeded warning threshold.
