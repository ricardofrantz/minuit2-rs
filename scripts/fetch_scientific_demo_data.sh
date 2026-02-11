#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DATA_DIR="${ROOT_DIR}/examples/data"

mkdir -p "${DATA_DIR}/noaa" "${DATA_DIR}/nist" "${DATA_DIR}/usgs" "${DATA_DIR}/cern"

echo "[1/4] NOAA CO2 monthly reference data"
curl -fLsS \
  "https://gml.noaa.gov/webdata/ccgg/trends/co2/co2_mm_mlo.csv" \
  -o "${DATA_DIR}/noaa/co2_mm_mlo.csv"

echo "[2/4] NIST StRD reference datasets"
curl -fLsS \
  "https://www.itl.nist.gov/div898/strd/nls/data/LINKS/DATA/Misra1a.dat" \
  -o "${DATA_DIR}/nist/Misra1a.dat"
curl -fLsS \
  "https://www.itl.nist.gov/div898/strd/nls/data/LINKS/DATA/Hahn1.dat" \
  -o "${DATA_DIR}/nist/Hahn1.dat"
curl -fLsS \
  "https://www.itl.nist.gov/div898/strd/nls/data/LINKS/DATA/Rat43.dat" \
  -o "${DATA_DIR}/nist/Rat43.dat"

echo "[3/4] USGS earthquake catalog snapshot (2025, M>=4.5)"
curl -fLsS \
  "https://earthquake.usgs.gov/fdsnws/event/1/query.csv?starttime=2025-01-01&endtime=2025-12-31&minmagnitude=4.5&orderby=time" \
  -o "${DATA_DIR}/usgs/earthquakes_2025_m4p5.csv"

echo "[4/4] CERN Open Data CSV references"
# CERN EOS public endpoint currently serves a cert chain that often requires -k
# in CI/dev shells without CERN CA roots.
curl -k -fLsS \
  "https://eospublic.cern.ch/eos/opendata/cms/Run2010B/Mu/CSV/Apr21ReReco-v1/MuRun2010B_0.csv" \
  -o "${DATA_DIR}/cern/MuRun2010B_0.csv"
curl -k -fLsS \
  "https://eospublic.cern.ch/eos/opendata/cms/Run2011A/DoubleMu/CSV/12Oct2013-v1/Zmumu.csv" \
  -o "${DATA_DIR}/cern/Zmumu.csv"

echo
echo "Downloaded files:"
wc -c \
  "${DATA_DIR}/noaa/co2_mm_mlo.csv" \
  "${DATA_DIR}/nist/Misra1a.dat" \
  "${DATA_DIR}/nist/Hahn1.dat" \
  "${DATA_DIR}/nist/Rat43.dat" \
  "${DATA_DIR}/usgs/earthquakes_2025_m4p5.csv" \
  "${DATA_DIR}/cern/MuRun2010B_0.csv" \
  "${DATA_DIR}/cern/Zmumu.csv"
