# Scientific Demo Options (C++ vs Rust)

Date: 2026-02-11

## Option 1: NOAA Mauna Loa CO2 Trend Fit (Recommended)
- Data:
  - `https://gml.noaa.gov/webdata/ccgg/trends/co2/co2_mm_mlo.csv`
- Use case:
  - Real atmospheric science time-series with uncertainty column.
- Curves/plots:
  - measured CO2 vs fitted curve
  - seasonal component
  - residuals vs time
  - runtime distribution (C++ vs Rust)
- Fit model:
  - polynomial trend + annual/semiannual harmonics
- Compare:
  - parameter agreement, chi2/ndf, covariance, runtime
- Effort/Risk:
  - Low effort, low risk

## Option 2: NIST StRD Nonlinear Regression Validation
- Data/docs:
  - `https://www.itl.nist.gov/div898/strd/nls/data/misra1a.shtml`
  - `https://www.itl.nist.gov/div898/strd/nls/data/ratkowsky3.shtml`
  - `https://www.nist.gov/itl/sed/products-services/statistical-reference-data-sets-strd`
- Use case:
  - Certified statistical reference datasets for validating optimization software.
- Curves/plots:
  - observed vs fitted curve (per dataset)
  - residuals
  - parameter error vs certified values
  - runtime and convergence distribution
- Compare:
  - C++ vs Rust agreement + error to NIST certified parameters
- Effort/Risk:
  - Medium effort, low risk

## Option 3: USGS Earthquake Magnitude-Frequency Fit
- Data/API:
  - `https://earthquake.usgs.gov/fdsnws/event/1/query.csv?...`
  - `https://earthquake.usgs.gov/fdsnws/event/1/schemas`
- Use case:
  - Real geophysics catalog; fit Gutenberg-Richter law.
- Curves/plots:
  - cumulative frequency vs magnitude
  - fitted line in log10 space
  - residuals
  - C++ vs Rust runtime comparison
- Compare:
  - slope/intercept agreement and fit quality
- Effort/Risk:
  - Low-medium effort, medium risk (data changes over time)

## Option 4: CERN Open Data Dimuon Spectrum
- Data:
  - `https://opendata.cern.ch/api/records/700`
  - `https://opendata.cern.ch/api/records/545`
- Use case:
  - HEP-aligned story: resonance peaks + background fit.
- Curves/plots:
  - invariant mass histogram
  - multi-peak + background fitted curve
  - residuals/pull distribution
  - C++ vs Rust parameter/runtime comparison
- Compare:
  - physics-meaningful parameters and covariance parity
- Effort/Risk:
  - Medium-high effort, high risk (xrootd/ingestion friction)

## Option 5: Two-Stage Showcase (Best Balance)
- Stage A:
  - NOAA CO2 (visual + real-world story)
- Stage B:
  - NIST StRD (certified correctness proof)
- Why:
  - Combines compelling plots with strong numerical credibility and manageable implementation cost.
