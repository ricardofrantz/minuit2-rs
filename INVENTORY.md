# Minuit2 File Inventory

Complete map of [GooFit/Minuit2](https://github.com/GooFit/Minuit2) files, categorized by phase and priority.

**Raw URL base**: `https://raw.githubusercontent.com/GooFit/Minuit2/master/`

## Legend

- **Port** = translate to Rust
- **Skip** = ROOT/Fumili/MPI adapter, not needed
- **Replace** = use `nalgebra` instead

---

## Phase 1 — Core Types & Infrastructure

### Function interfaces
| File | Action | Notes |
|------|--------|-------|
| [inc/Minuit2/FCNBase.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FCNBase.h) | Port | → `trait FCN` |
| [inc/Minuit2/FCNGradientBase.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FCNGradientBase.h) | Port | → `trait FCNGradient: FCN` |
| [inc/Minuit2/GenericFunction.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/GenericFunction.h) | Port | Base function type |
| [inc/Minuit2/FCNAdapter.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FCNAdapter.h) | Skip | ROOT Math adapter |
| [inc/Minuit2/FCNGradAdapter.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FCNGradAdapter.h) | Skip | ROOT Math adapter |

### Parameter management
| File | Action |
|------|--------|
| [inc/Minuit2/MinuitParameter.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinuitParameter.h) | Port |
| [inc/Minuit2/MnUserParameters.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnUserParameters.h) | Port |
| [src/MnUserParameters.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnUserParameters.cxx) | Port |
| [inc/Minuit2/MnUserParameterState.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnUserParameterState.h) | Port |
| [src/MnUserParameterState.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnUserParameterState.cxx) | Port |
| [inc/Minuit2/MnUserCovariance.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnUserCovariance.h) | Port |
| [inc/Minuit2/MnUserTransformation.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnUserTransformation.h) | Port |
| [src/MnUserTransformation.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnUserTransformation.cxx) | Port |
| [inc/Minuit2/MnUserFcn.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnUserFcn.h) | Port |
| [src/MnUserFcn.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnUserFcn.cxx) | Port |

### Parameter transforms (bounded → unbounded)
| File | Action |
|------|--------|
| [inc/Minuit2/SinParameterTransformation.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SinParameterTransformation.h) | Port |
| [src/SinParameterTransformation.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SinParameterTransformation.cxx) | Port |
| [inc/Minuit2/SqrtLowParameterTransformation.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SqrtLowParameterTransformation.h) | Port |
| [src/SqrtLowParameterTransformation.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SqrtLowParameterTransformation.cxx) | Port |
| [inc/Minuit2/SqrtUpParameterTransformation.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SqrtUpParameterTransformation.h) | Port |
| [src/SqrtUpParameterTransformation.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SqrtUpParameterTransformation.cxx) | Port |
| [inc/Minuit2/MnVectorTransform.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnVectorTransform.h) | Port |

### Result types
| File | Action |
|------|--------|
| [inc/Minuit2/FunctionMinimum.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FunctionMinimum.h) | Port |
| [inc/Minuit2/BasicFunctionMinimum.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicFunctionMinimum.h) | Port |
| [inc/Minuit2/MinimumState.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumState.h) | Port |
| [inc/Minuit2/BasicMinimumState.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicMinimumState.h) | Port |
| [inc/Minuit2/MinimumParameters.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumParameters.h) | Port |
| [inc/Minuit2/BasicMinimumParameters.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicMinimumParameters.h) | Port |
| [inc/Minuit2/MinimumError.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumError.h) | Port |
| [inc/Minuit2/BasicMinimumError.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicMinimumError.h) | Port |
| [src/BasicMinimumError.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/BasicMinimumError.cxx) | Port |
| [inc/Minuit2/MinimumSeed.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumSeed.h) | Port |
| [inc/Minuit2/BasicMinimumSeed.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicMinimumSeed.h) | Port |
| [inc/Minuit2/FunctionGradient.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FunctionGradient.h) | Port |
| [inc/Minuit2/BasicFunctionGradient.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BasicFunctionGradient.h) | Port |

### Strategy & config
| File | Action |
|------|--------|
| [inc/Minuit2/MnStrategy.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnStrategy.h) | Port |
| [src/MnStrategy.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnStrategy.cxx) | Port |
| [inc/Minuit2/MnMachinePrecision.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnMachinePrecision.h) | Port |
| [src/MnMachinePrecision.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnMachinePrecision.cxx) | Port |
| [inc/Minuit2/MnTiny.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnTiny.h) | Port |
| [src/MnTiny.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnTiny.cxx) | Port |
| [inc/Minuit2/MnConfig.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnConfig.h) | Port |

### Minimizer framework (abstract base)
| File | Action |
|------|--------|
| [inc/Minuit2/MnApplication.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnApplication.h) | Port |
| [src/MnApplication.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnApplication.cxx) | Port |
| [inc/Minuit2/FunctionMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/FunctionMinimizer.h) | Port |
| [inc/Minuit2/ModularFunctionMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ModularFunctionMinimizer.h) | Port |
| [src/ModularFunctionMinimizer.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/ModularFunctionMinimizer.cxx) | Port |
| [inc/Minuit2/MinimumBuilder.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumBuilder.h) | Port |
| [src/MinimumBuilder.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MinimumBuilder.cxx) | Port |
| [inc/Minuit2/MinimumSeedGenerator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumSeedGenerator.h) | Port |
| [inc/Minuit2/MinimumErrorUpdator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinimumErrorUpdator.h) | Port |
| [inc/Minuit2/GradientCalculator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/GradientCalculator.h) | Port |
| [inc/Minuit2/MnFcn.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnFcn.h) | Port |
| [src/MnFcn.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnFcn.cxx) | Port |

### Printing/tracing
| File | Action |
|------|--------|
| [inc/Minuit2/MnPrint.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnPrint.h) | Port |
| [src/MnPrint.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnPrint.cxx) | Port |
| [src/MnPrintImpl.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnPrintImpl.cxx) | Port |
| [inc/Minuit2/MnTraceObject.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnTraceObject.h) | Port |
| [src/MnTraceObject.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnTraceObject.cxx) | Port |

### Linear algebra (replace with `nalgebra`)
| File | Action | Notes |
|------|--------|-------|
| [inc/Minuit2/LASymMatrix.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/LASymMatrix.h) | Replace | → `nalgebra::DMatrix<f64>` |
| [inc/Minuit2/LAVector.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/LAVector.h) | Replace | → `nalgebra::DVector<f64>` |
| [inc/Minuit2/MnMatrix.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnMatrix.h) | Replace | Type aliases |
| [inc/Minuit2/ABObj.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ABObj.h) | Replace | Expression templates → nalgebra ops |
| [inc/Minuit2/ABProd.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ABProd.h) | Replace | |
| [inc/Minuit2/ABSum.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ABSum.h) | Replace | |
| [inc/Minuit2/ABTypes.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ABTypes.h) | Replace | |
| [inc/Minuit2/LaInverse.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/LaInverse.h) | Replace | |
| [inc/Minuit2/LaOuterProduct.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/LaOuterProduct.h) | Replace | |
| [inc/Minuit2/LaProd.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/LaProd.h) | Replace | |
| [inc/Minuit2/LaSum.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/LaSum.h) | Replace | |
| [inc/Minuit2/MatrixInverse.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MatrixInverse.h) | Replace | |
| [inc/Minuit2/VectorOuterProduct.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/VectorOuterProduct.h) | Replace | |
| [inc/Minuit2/StackAllocator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/StackAllocator.h) | Skip | Custom allocator, not needed in Rust |
| [inc/Minuit2/MnRefCountedPointer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnRefCountedPointer.h) | Skip | → `Arc`/`Rc` |
| [inc/Minuit2/MnReferenceCounter.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnReferenceCounter.h) | Skip | → `Arc`/`Rc` |

### BLAS-like routines (replace with `nalgebra`)
| File | Action |
|------|--------|
| [src/LaEigenValues.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/LaEigenValues.cxx) | Replace |
| [src/LaInnerProduct.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/LaInnerProduct.cxx) | Replace |
| [src/LaInverse.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/LaInverse.cxx) | Replace |
| [src/LaOuterProduct.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/LaOuterProduct.cxx) | Replace |
| [src/LaSumOfElements.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/LaSumOfElements.cxx) | Replace |
| [src/LaVtMVSimilarity.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/LaVtMVSimilarity.cxx) | Replace |
| [src/mndasum.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mndasum.cxx) | Replace |
| [src/mndaxpy.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mndaxpy.cxx) | Replace |
| [src/mnddot.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mnddot.cxx) | Replace |
| [src/mndscal.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mndscal.cxx) | Replace |
| [src/mndspmv.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mndspmv.cxx) | Replace |
| [src/mndspr.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mndspr.cxx) | Replace |
| [src/mnlsame.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mnlsame.cxx) | Replace |
| [src/mnvert.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mnvert.cxx) | Replace |
| [src/mnxerbla.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mnxerbla.cxx) | Replace |

---

## Phase 2 — Simplex

| File | Action |
|------|--------|
| [inc/Minuit2/MnSimplex.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnSimplex.h) | Port |
| [inc/Minuit2/SimplexMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SimplexMinimizer.h) | Port |
| [inc/Minuit2/SimplexBuilder.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SimplexBuilder.h) | Port |
| [src/SimplexBuilder.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SimplexBuilder.cxx) | Port |
| [inc/Minuit2/SimplexParameters.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SimplexParameters.h) | Port |
| [src/SimplexParameters.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SimplexParameters.cxx) | Port |
| [inc/Minuit2/SimplexSeedGenerator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/SimplexSeedGenerator.h) | Port |
| [src/SimplexSeedGenerator.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/SimplexSeedGenerator.cxx) | Port |

---

## Phase 3 — Migrad

### Core iteration
| File | Action |
|------|--------|
| [inc/Minuit2/MnMigrad.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnMigrad.h) | Port |
| [inc/Minuit2/VariableMetricMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/VariableMetricMinimizer.h) | Port |
| [inc/Minuit2/VariableMetricBuilder.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/VariableMetricBuilder.h) | Port |
| [src/VariableMetricBuilder.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/VariableMetricBuilder.cxx) | Port |
| [inc/Minuit2/VariableMetricEDMEstimator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/VariableMetricEDMEstimator.h) | Port |
| [src/VariableMetricEDMEstimator.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/VariableMetricEDMEstimator.cxx) | Port |

### Gradient calculators
| File | Action |
|------|--------|
| [inc/Minuit2/Numerical2PGradientCalculator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/Numerical2PGradientCalculator.h) | Port |
| [src/Numerical2PGradientCalculator.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/Numerical2PGradientCalculator.cxx) | Port |
| [inc/Minuit2/InitialGradientCalculator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/InitialGradientCalculator.h) | Port |
| [src/InitialGradientCalculator.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/InitialGradientCalculator.cxx) | Port |
| [inc/Minuit2/HessianGradientCalculator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/HessianGradientCalculator.h) | Port |
| [src/HessianGradientCalculator.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/HessianGradientCalculator.cxx) | Port |
| [inc/Minuit2/AnalyticalGradientCalculator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/AnalyticalGradientCalculator.h) | Port |
| [src/AnalyticalGradientCalculator.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/AnalyticalGradientCalculator.cxx) | Port |

### Line search
| File | Action |
|------|--------|
| [inc/Minuit2/MnLineSearch.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnLineSearch.h) | Port |
| [src/MnLineSearch.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnLineSearch.cxx) | Port |
| [inc/Minuit2/MnParabola.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnParabola.h) | Port |
| [inc/Minuit2/MnParabolaFactory.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnParabolaFactory.h) | Port |
| [src/MnParabolaFactory.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnParabolaFactory.cxx) | Port |
| [inc/Minuit2/MnParabolaPoint.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnParabolaPoint.h) | Port |

### Hessian updators
| File | Action |
|------|--------|
| [inc/Minuit2/DavidonErrorUpdator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/DavidonErrorUpdator.h) | Port |
| [src/DavidonErrorUpdator.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/DavidonErrorUpdator.cxx) | Port |
| [inc/Minuit2/BFGSErrorUpdator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/BFGSErrorUpdator.h) | Port |
| [src/BFGSErrorUpdator.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/BFGSErrorUpdator.cxx) | Port |

### Seed generator & helpers
| File | Action |
|------|--------|
| [inc/Minuit2/MnSeedGenerator.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnSeedGenerator.h) | Port |
| [src/MnSeedGenerator.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnSeedGenerator.cxx) | Port |
| [inc/Minuit2/NegativeG2LineSearch.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/NegativeG2LineSearch.h) | Port |
| [src/NegativeG2LineSearch.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/NegativeG2LineSearch.cxx) | Port |
| [inc/Minuit2/MnPosDef.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnPosDef.h) | Port |
| [src/MnPosDef.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnPosDef.cxx) | Port |
| [inc/Minuit2/MnEigen.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnEigen.h) | Port |
| [src/MnEigen.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnEigen.cxx) | Port |
| [src/mnteigen.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mnteigen.cxx) | Port |

---

## Phase 4 — Error Analysis

### Hesse
| File | Action |
|------|--------|
| [inc/Minuit2/MnHesse.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnHesse.h) | Port |
| [src/MnHesse.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnHesse.cxx) | Port |
| [inc/Minuit2/MnCovarianceSqueeze.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnCovarianceSqueeze.h) | Port |
| [src/MnCovarianceSqueeze.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnCovarianceSqueeze.cxx) | Port |
| [inc/Minuit2/MnGlobalCorrelationCoeff.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnGlobalCorrelationCoeff.h) | Port |
| [src/MnGlobalCorrelationCoeff.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnGlobalCorrelationCoeff.cxx) | Port |

### Minos
| File | Action |
|------|--------|
| [inc/Minuit2/MnMinos.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnMinos.h) | Port |
| [src/MnMinos.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnMinos.cxx) | Port |
| [inc/Minuit2/MinosError.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MinosError.h) | Port |
| [inc/Minuit2/MnCross.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnCross.h) | Port |
| [inc/Minuit2/MnFunctionCross.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnFunctionCross.h) | Port |
| [src/MnFunctionCross.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnFunctionCross.cxx) | Port |

### Scan & Contours
| File | Action |
|------|--------|
| [inc/Minuit2/MnScan.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnScan.h) | Port |
| [src/MnScan.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnScan.cxx) | Port |
| [inc/Minuit2/MnParameterScan.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnParameterScan.h) | Port |
| [src/MnParameterScan.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnParameterScan.cxx) | Port |
| [inc/Minuit2/ScanBuilder.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ScanBuilder.h) | Port |
| [src/ScanBuilder.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/ScanBuilder.cxx) | Port |
| [inc/Minuit2/ScanMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ScanMinimizer.h) | Port |
| [inc/Minuit2/MnContours.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnContours.h) | Port |
| [src/MnContours.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnContours.cxx) | Port |
| [inc/Minuit2/ContoursError.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/ContoursError.h) | Port |

---

## Phase 5 — Extended & Stretch

### Combined minimizer (Simplex → Migrad)
| File | Action |
|------|--------|
| [inc/Minuit2/MnMinimize.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnMinimize.h) | Port |
| [inc/Minuit2/CombinedMinimizer.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/CombinedMinimizer.h) | Port |
| [inc/Minuit2/CombinedMinimumBuilder.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/CombinedMinimumBuilder.h) | Port |
| [src/CombinedMinimumBuilder.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/CombinedMinimumBuilder.cxx) | Port |

### Plotting (optional)
| File | Action |
|------|--------|
| [inc/Minuit2/MnPlot.h](https://raw.githubusercontent.com/GooFit/Minuit2/master/inc/Minuit2/MnPlot.h) | Optional |
| [src/MnPlot.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/MnPlot.cxx) | Optional |
| [src/mnbins.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mnbins.cxx) | Optional |
| [src/mntplot.cxx](https://raw.githubusercontent.com/GooFit/Minuit2/master/src/mntplot.cxx) | Optional |

---

## Skip — ROOT/Fumili/MPI (not needed)

| File | Reason |
|------|--------|
| `inc/Math/*.h` (11 files) | ROOT Math interface |
| `inc/Fit/ParameterSettings.h` | ROOT Fit interface |
| `inc/LinkDef.h` | ROOT dictionary generator |
| `inc/TMinuit2TraceObject.h`, `src/TMinuit2TraceObject.cxx` | ROOT TObject |
| `inc/Minuit2/Minuit2Minimizer.h`, `src/Minuit2Minimizer.cxx` | ROOT Math::Minimizer adapter |
| `inc/Minuit2/Fumili*.h` (10 files), `src/Fumili*.cxx` (5 files) | Fumili method (niche, add later) |
| `inc/Minuit2/MPIProcess.h`, `src/MPIProcess.cxx` | MPI parallelism |
| `inc/Minuit2/ParametricFunction.h`, `src/ParametricFunction.cxx` | Fumili dependency |
| `src/math/*.cxx` (2 files) | ROOT Math options |
| `src/FitterUtil.h` | Internal ROOT util |

---

## Summary

| Category | Files | Action |
|----------|-------|--------|
| Core to port (Phase 1-4) | ~95 | Translate |
| Replace with nalgebra | ~28 | Not translated, use crate |
| Skip (ROOT/Fumili/MPI) | ~34 | Ignore |
| Optional (Phase 5) | ~8 | Low priority |
| **Total** | **~165** | |
