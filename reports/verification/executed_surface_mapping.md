# Executed Surface Mapping

Join of reference executed C++ functions with traceability matrix mappings.

## Summary

- Executed C++ functions: **618**
- Mapped to implemented Rust symbols: **68**
- Unmapped executed functions: **550**
- Unmapped priority split: P0=0, P1=139, P2=411
- Gate (`P0 == 0 and P1 == 0`): **FAIL**
- Coverage workloads used: **12**

## Artifacts

- `reports/verification/executed_surface_mapping.md`
- `reports/verification/executed_surface_gaps.csv`
- `reports/verification/executed_surface_manifest.json`

## Top Gap Files

- `inc/Minuit2/MnPrint.h`: 220
- `inc/Minuit2/MnMatrix.h`: 75
- `inc/ROOT/span.hxx`: 22
- `src/MnUserParameterState.cxx`: 22
- `src/MnUserTransformation.cxx`: 19
- `src/MnMatrix.cxx`: 14
- `src/MnUserParameters.cxx`: 13
- `src/MnPrint.cxx`: 9
- `inc/Minuit2/FunctionGradient.h`: 8
- `inc/Minuit2/MinimumSeed.h`: 6
- `src/MnHesse.cxx`: 6
- `src/MnMinos.cxx`: 6
- `inc/Minuit2/MinimumParameters.h`: 5
- `src/AnalyticalGradientCalculator.cxx`: 5
- `src/MPIProcess.h`: 5

## Top P0/P1 Gaps

| Priority | Upstream file | Symbol | Mapping status | Call count |
|---|---|---|---|---|
| P1 | `inc/Minuit2/FunctionGradient.h` | `G2` | `missing` | 206 |
| P1 | `inc/Minuit2/FunctionGradient.h` | `Grad` | `missing` | 992 |
| P1 | `inc/Minuit2/FunctionGradient.h` | `Gstep` | `missing` | 169 |
| P1 | `inc/Minuit2/FunctionGradient.h` | `Vec` | `missing` | 600 |
| P1 | `inc/Minuit2/FunctionMinimum.h` | `Error` | `missing` | 29 |
| P1 | `inc/Minuit2/FunctionMinimum.h` | `Parameters` | `missing` | 8 |
| P1 | `inc/Minuit2/HessianGradientCalculator.h` | `Fcn` | `missing` | 15 |
| P1 | `inc/Minuit2/HessianGradientCalculator.h` | `Strategy` | `missing` | 30 |
| P1 | `inc/Minuit2/MinimumError.h` | `InvHessian` | `missing` | 672 |
| P1 | `inc/Minuit2/MinimumParameters.h` | `Dirin` | `missing` | 2 |
| P1 | `inc/Minuit2/MinimumParameters.h` | `Vec` | `missing` | 1474 |
| P1 | `inc/Minuit2/MinimumSeed.h` | `Error` | `missing` | 59 |
| P1 | `inc/Minuit2/MinimumSeed.h` | `Gradient` | `missing` | 60 |
| P1 | `inc/Minuit2/MinimumSeed.h` | `Parameters` | `missing` | 146 |
| P1 | `inc/Minuit2/MinimumSeed.h` | `Precision` | `missing` | 28 |
| P1 | `inc/Minuit2/MinimumSeed.h` | `State` | `missing` | 381 |
| P1 | `inc/Minuit2/MinimumState.h` | `Gradient` | `missing` | 735 |
| P1 | `inc/Minuit2/MinimumState.h` | `Vec` | `missing` | 366 |
| P1 | `inc/Minuit2/MinuitParameter.h` | `GetName` | `missing` | 484 |
| P1 | `inc/Minuit2/MnFcn.h` | `Fcn` | `missing` | 853 |
| P1 | `inc/Minuit2/MnFcn.h` | `Trafo` | `missing` | 1334 |
| P1 | `inc/Minuit2/MnUserParameterState.h` | `IntCovariance` | `missing` | 32 |
| P1 | `inc/Minuit2/MnUserParameterState.h` | `IntParameters` | `missing` | 38 |
| P1 | `inc/Minuit2/MnUserParameterState.h` | `Parameters` | `missing` | 45 |
| P1 | `inc/Minuit2/MnUserParameterState.h` | `Trafo` | `missing` | 128 |
| P1 | `inc/Minuit2/MnUserParameters.h` | `Trafo` | `missing` | 417 |
| P1 | `inc/Minuit2/MnUserTransformation.h` | `InitialParValues` | `missing` | 488 |
| P1 | `inc/Minuit2/MnUserTransformation.h` | `Parameters` | `missing` | 94 |
| P1 | `inc/Minuit2/MnUserTransformation.h` | `Precision` | `missing` | 553 |
| P1 | `inc/Minuit2/VariableMetricBuilder.h` | `ErrorUpdator` | `missing` | 81 |
| P1 | `inc/Minuit2/VariableMetricBuilder.h` | `Estimator` | `missing` | 81 |
| P1 | `inc/ROOT/span.hxx` | `begin` | `missing` | 87 |
| P1 | `inc/ROOT/span.hxx` | `begin` | `missing` | 1 |
| P1 | `inc/ROOT/span.hxx` | `begin` | `missing` | 1 |
| P1 | `inc/ROOT/span.hxx` | `begin` | `missing` | 55 |
| P1 | `inc/ROOT/span.hxx` | `data` | `missing` | 6574 |
| P1 | `inc/ROOT/span.hxx` | `end` | `missing` | 87 |
| P1 | `inc/ROOT/span.hxx` | `end` | `missing` | 1 |
| P1 | `inc/ROOT/span.hxx` | `end` | `missing` | 1 |
| P1 | `inc/ROOT/span.hxx` | `end` | `missing` | 55 |

