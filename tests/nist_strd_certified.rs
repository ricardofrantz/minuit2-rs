//! NIST StRD certified-oracle integration tests.
//!
//! These tests fit official NIST Statistical Reference Datasets (StRD) for
//! nonlinear least-squares regression and assert that MnMigrad converges to the
//! NIST-CERTIFIED parameter values within realistic relative tolerances.
//!
//! The oracle (certified parameter values, recommended starting values, and the
//! observed data) is PARSED AT TEST TIME from the committed `.dat` files under
//! `examples/data/nist/`. Nothing is fabricated here — the numbers come from
//! NIST. Each `.dat` file is pinned in `examples/data/SHA256SUMS` and fetched/
//! verified by `scripts/fetch_scientific_demo_data.sh`.
//!
//! Source: https://www.itl.nist.gov/div898/strd/nls/data/LINKS/DATA/<NAME>.dat
//!
//! Fits start from the NIST "Start 2" values (the harder of the two recommended
//! starts) and use a plain least-squares FCN with errordef/up = 1.0.
//!
//! Per-dataset tolerances are chosen by NIST difficulty tier:
//!   - Lower / average difficulty      -> ~1e-3 relative
//!   - Higher difficulty / ill-conditioned -> ~1e-2 relative (looser, never weaker than needed)
//!
//! The exact tolerance is stated in a comment at each dataset's test.
//!
//! Run:
//!   cargo test --test nist_strd_certified -- --nocapture

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use minuit2::{FCN, MnMigrad};

/// Resolve a path relative to the crate manifest directory.
fn repo_path(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// Parsed NIST StRD dataset.
#[derive(Clone)]
struct NistDataset {
    name: String,
    x: Vec<f64>,
    y: Vec<f64>,
    #[allow(dead_code)]
    start1: Vec<f64>,
    start2: Vec<f64>,
    certified: Vec<f64>,
    #[allow(dead_code)]
    certified_rss: f64,
}

/// Plain least-squares FCN: sum of squared residuals, errordef = 1.0.
struct LeastSquaresFCN {
    x: Vec<f64>,
    y: Vec<f64>,
    model: fn(&[f64], f64) -> f64,
}

impl FCN for LeastSquaresFCN {
    fn value(&self, p: &[f64]) -> f64 {
        let mut rss = 0.0;
        for i in 0..self.x.len() {
            let pred = (self.model)(p, self.x[i]);
            if !pred.is_finite() {
                return 1e30;
            }
            let r = self.y[i] - pred;
            rss += r * r;
        }
        rss
    }

    fn error_def(&self) -> f64 {
        1.0
    }
}

/// Parse a single floating-point token from a NIST `.dat` file, tolerating the
/// Fortran-style `E0` exponents and bare-leading-dot forms NIST uses.
fn parse_float_token(token: &str) -> Option<f64> {
    let t = token.trim().trim_end_matches(',');
    if t.is_empty() {
        return None;
    }
    if let Ok(v) = t.parse::<f64>() {
        return Some(v);
    }
    if let Some(rest) = t.strip_prefix('.') {
        return format!("0.{rest}").parse::<f64>().ok();
    }
    if let Some(rest) = t.strip_prefix("-.") {
        return format!("-0.{rest}").parse::<f64>().ok();
    }
    None
}

fn parse_floats(text: &str) -> Vec<f64> {
    text.split_whitespace()
        .filter_map(parse_float_token)
        .collect()
}

/// Parse a committed NIST StRD `.dat` file.
///
/// Extracts the dataset name, Start 1 / Start 2 values, certified parameter
/// estimates, certified residual sum of squares, and the `(x, y)` observations.
/// The certified values are the test oracle and live only in the file.
fn parse_nist_dat(path: &Path, expected_params: usize) -> Result<NistDataset, String> {
    let f = File::open(path).map_err(|e| format!("failed to open {}: {e}", path.display()))?;
    let reader = BufReader::new(f);

    let mut name = String::new();
    let mut start1 = Vec::new();
    let mut start2 = Vec::new();
    let mut certified = Vec::new();
    let mut certified_rss = f64::NAN;
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut in_data = false;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("error reading {}: {e}", path.display()))?;
        let s = line.trim();

        if s.starts_with("Dataset Name:") {
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() >= 3 {
                name = parts[2].to_string();
            }
        }

        // Parameter lines look like: `b1 =   500   250   2.38...E+02   2.70...E+00`
        if let Some((lhs, rhs)) = s.split_once('=') {
            let lhs = lhs.trim_start();
            // Match `b<digit>` only, so model-equation lines never qualify.
            let is_b_param = lhs.starts_with('b')
                && lhs.len() >= 2
                && lhs[1..].chars().next().is_some_and(|c| c.is_ascii_digit());
            if is_b_param {
                let nums = parse_floats(rhs);
                if nums.len() >= 4 {
                    start1.push(nums[0]);
                    start2.push(nums[1]);
                    certified.push(nums[2]);
                }
            }
        }

        if s.starts_with("Residual Sum of Squares:") {
            let nums = parse_floats(s);
            if let Some(v) = nums.last() {
                certified_rss = *v;
            }
        }

        // The observed data block begins at the second `Data:` header, whose
        // tail starts with the response-variable name `y`.
        if let Some(tail) = s.strip_prefix("Data:") {
            let tail = tail.trim_start();
            if tail.starts_with('y') {
                in_data = true;
                continue;
            }
        }

        if in_data {
            let nums = parse_floats(s);
            if nums.len() >= 2 {
                y.push(nums[0]);
                x.push(nums[1]);
            }
        }
    }

    if name.is_empty() {
        name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("dataset")
            .to_string();
    }
    if start1.len() != expected_params
        || start2.len() != expected_params
        || certified.len() != expected_params
    {
        return Err(format!(
            "{name}: expected {expected_params} parameters, parsed \
             start1={}, start2={}, certified={}",
            start1.len(),
            start2.len(),
            certified.len()
        ));
    }
    if x.is_empty() {
        return Err(format!(
            "{name}: no data rows parsed from {}",
            path.display()
        ));
    }

    Ok(NistDataset {
        name,
        x,
        y,
        start1,
        start2,
        certified,
        certified_rss,
    })
}

/// Fit `model` to `ds` starting from `starts`, returning the fitted parameters.
///
/// Optional per-parameter lower bounds are applied at the given indices to keep
/// the search in a physically sensible region (only used where NIST models have
/// a divide-by-zero / negative-power singularity off-region).
fn fit(
    ds: &NistDataset,
    model: fn(&[f64], f64) -> f64,
    starts: &[f64],
    lower_limited: &[(usize, f64)],
    tolerance: f64,
) -> (Vec<f64>, bool, f64) {
    let fcn = LeastSquaresFCN {
        x: ds.x.clone(),
        y: ds.y.clone(),
        model,
    };

    let mut migrad = MnMigrad::new();
    for (i, start) in starts.iter().enumerate() {
        let pname = format!("b{}", i + 1);
        let step = (start.abs() * 0.1).max(1e-4);
        if let Some(&(_, lo)) = lower_limited.iter().find(|(idx, _)| *idx == i) {
            migrad = migrad.add_lower_limited(&pname, *start, step, lo);
        } else {
            migrad = migrad.add(&pname, *start, step);
        }
    }

    let min = migrad
        .with_strategy(2)
        .tolerance(tolerance)
        .max_fcn(1_000_000)
        .minimize(&fcn);

    (min.params(), min.is_valid(), min.fval())
}

/// Assert each fitted parameter is within `rel_tol` (relative) of the certified
/// NIST value. Prints a per-parameter delta table for `--nocapture` visibility.
fn assert_certified(ds: &NistDataset, fitted: &[f64], rel_tol: f64) {
    println!("=== {} (rel_tol = {rel_tol:.0e}) ===", ds.name);
    println!("points: {}", ds.x.len());
    let mut worst_rel = 0.0_f64;
    for (i, (&cert, &fit)) in ds.certified.iter().zip(fitted.iter()).enumerate() {
        let abs_err = (fit - cert).abs();
        let rel_err = abs_err / cert.abs().max(1e-300);
        worst_rel = worst_rel.max(rel_err);
        println!(
            "  b{} = {:>16.8e} | cert {:>16.8e} | abs {:>10.3e} | rel {:>10.3e}",
            i + 1,
            fit,
            cert,
            abs_err,
            rel_err
        );
    }
    println!("  worst relative error: {worst_rel:.3e}\n");

    for (i, (&cert, &fit)) in ds.certified.iter().zip(fitted.iter()).enumerate() {
        let rel_err = (fit - cert).abs() / cert.abs().max(1e-300);
        assert!(
            rel_err <= rel_tol,
            "{}: b{} fitted {:.10e} vs certified {:.10e}, relative error {:.3e} exceeds tol {:.0e}",
            ds.name,
            i + 1,
            fit,
            cert,
            rel_err,
            rel_tol
        );
    }
}

fn load(name: &str, nparams: usize) -> NistDataset {
    let path = repo_path(&format!("examples/data/nist/{name}.dat"));
    parse_nist_dat(&path, nparams)
        .unwrap_or_else(|e| panic!("failed to parse {}: {e}", path.display()))
}

// ---------------------------------------------------------------------------
// Models (transcribed verbatim from each `.dat` file's Model: section).
// ---------------------------------------------------------------------------

// Misra1a / BoxBOD: y = b1*(1 - exp(-b2*x))
fn model_misra1a(p: &[f64], x: f64) -> f64 {
    p[0] * (1.0 - (-p[1] * x).exp())
}

// Misra1b: y = b1 * (1 - (1 + b2*x/2)^(-2))
fn model_misra1b(p: &[f64], x: f64) -> f64 {
    let base = 1.0 + p[1] * x / 2.0;
    p[0] * (1.0 - base.powi(-2))
}

// Chwirut2: y = exp(-b1*x) / (b2 + b3*x)
fn model_chwirut2(p: &[f64], x: f64) -> f64 {
    let den = p[1] + p[2] * x;
    if den.abs() < 1e-300 {
        return f64::NAN;
    }
    (-p[0] * x).exp() / den
}

// Rat42: y = b1 / (1 + exp(b2 - b3*x))
fn model_rat42(p: &[f64], x: f64) -> f64 {
    let expo = (p[1] - p[2] * x).clamp(-700.0, 700.0);
    p[0] / (1.0 + expo.exp())
}

// Kirby2: y = (b1 + b2*x + b3*x^2) / (1 + b4*x + b5*x^2)
fn model_kirby2(p: &[f64], x: f64) -> f64 {
    let x2 = x * x;
    let den = 1.0 + p[3] * x + p[4] * x2;
    if den.abs() < 1e-300 {
        return f64::NAN;
    }
    (p[0] + p[1] * x + p[2] * x2) / den
}

// Thurber: y = (b1 + b2*x + b3*x^2 + b4*x^3) / (1 + b5*x + b6*x^2 + b7*x^3)
fn model_thurber(p: &[f64], x: f64) -> f64 {
    let x2 = x * x;
    let x3 = x2 * x;
    let num = p[0] + p[1] * x + p[2] * x2 + p[3] * x3;
    let den = 1.0 + p[4] * x + p[5] * x2 + p[6] * x3;
    if den.abs() < 1e-300 {
        return f64::NAN;
    }
    num / den
}

// Gauss1: y = b1*exp(-b2*x) + b3*exp(-(x-b4)^2/b5^2) + b6*exp(-(x-b7)^2/b8^2)
fn model_gauss1(p: &[f64], x: f64) -> f64 {
    let g1 = p[0] * (-p[1] * x).exp();
    let g2 = p[2] * (-((x - p[3]).powi(2)) / (p[4] * p[4])).exp();
    let g3 = p[5] * (-((x - p[6]).powi(2)) / (p[7] * p[7])).exp();
    g1 + g2 + g3
}

// ENSO: y = b1 + b2*cos(2*pi*x/12) + b3*sin(2*pi*x/12)
//          + b5*cos(2*pi*x/b4) + b6*sin(2*pi*x/b4)
//          + b8*cos(2*pi*x/b7) + b9*sin(2*pi*x/b7)
fn model_enso(p: &[f64], x: f64) -> f64 {
    use std::f64::consts::PI;
    let w = 2.0 * PI * x;
    p[0] + p[1] * (w / 12.0).cos()
        + p[2] * (w / 12.0).sin()
        + p[4] * (w / p[3]).cos()
        + p[5] * (w / p[3]).sin()
        + p[7] * (w / p[6]).cos()
        + p[8] * (w / p[6]).sin()
}

// ---------------------------------------------------------------------------
// Tests. Each runs MnMigrad from NIST "Start 2".
// ---------------------------------------------------------------------------

// Lower difficulty. Tolerance 1e-3.
#[test]
fn nist_misra1a() {
    let ds = load("Misra1a", 2);
    let (p, valid, _) = fit(&ds, model_misra1a, &ds.start2, &[], 1e-4);
    assert!(valid, "Misra1a: migrad should converge");
    assert_certified(&ds, &p, 1e-3);
}

// Lower difficulty. Tolerance 1e-3.
#[test]
fn nist_misra1b() {
    let ds = load("Misra1b", 2);
    let (p, valid, _) = fit(&ds, model_misra1b, &ds.start2, &[], 1e-4);
    assert!(valid, "Misra1b: migrad should converge");
    assert_certified(&ds, &p, 1e-3);
}

// Lower difficulty. Tolerance 1e-3.
#[test]
fn nist_chwirut2() {
    let ds = load("Chwirut2", 3);
    let (p, valid, _) = fit(&ds, model_chwirut2, &ds.start2, &[], 1e-4);
    assert!(valid, "Chwirut2: migrad should converge");
    assert_certified(&ds, &p, 1e-3);
}

// Average difficulty. Tolerance 1e-3.
#[test]
fn nist_rat42() {
    let ds = load("Rat42", 3);
    let (p, valid, _) = fit(&ds, model_rat42, &ds.start2, &[], 1e-4);
    assert!(valid, "Rat42: migrad should converge");
    assert_certified(&ds, &p, 1e-3);
}

// Higher difficulty (rational, ill-conditioned). Tolerance 1e-2.
#[test]
fn nist_kirby2() {
    let ds = load("Kirby2", 5);
    let (p, valid, _) = fit(&ds, model_kirby2, &ds.start2, &[], 1e-5);
    assert!(valid, "Kirby2: migrad should converge");
    assert_certified(&ds, &p, 1e-2);
}

// Higher difficulty (rational cubic/cubic). Tolerance 1e-2.
#[test]
fn nist_thurber() {
    let ds = load("Thurber", 7);
    let (p, valid, _) = fit(&ds, model_thurber, &ds.start2, &[], 1e-5);
    assert!(valid, "Thurber: migrad should converge");
    assert_certified(&ds, &p, 1e-2);
}

// Higher difficulty (three-Gaussian sum). Tolerance 1e-2.
#[test]
fn nist_gauss1() {
    let ds = load("Gauss1", 8);
    let (p, valid, _) = fit(&ds, model_gauss1, &ds.start2, &[], 1e-4);
    assert!(valid, "Gauss1: migrad should converge");
    assert_certified(&ds, &p, 1e-2);
}

// Higher difficulty (seasonal + period harmonics). Tolerance 1e-2.
#[test]
fn nist_enso() {
    let ds = load("ENSO", 9);
    let (p, valid, _) = fit(&ds, model_enso, &ds.start2, &[], 1e-4);
    assert!(valid, "ENSO: migrad should converge");
    assert_certified(&ds, &p, 1e-2);
}

// NOTE on SKIPPED datasets (curated for honesty, NOT forced to pass by weakening
// tolerances). Each is omitted because plain MnMigrad from NIST "Start 2" does
// not reach the certified global minimum without extra machinery, and masking a
// non-converged fit with a loose tolerance would defeat the purpose of an oracle
// test. The bundled `examples/nist_strd` demo handles such cases with multistart
// search and parameter rescaling.
//
// - Lanczos3 (certified RSS = 1.6e-8, an essentially exact fit): the three
//   exponential terms are extremely correlated. From Start 2 plain MnMigrad lands
//   in a different basin (finite RSS, swapped/merged components) rather than the
//   certified parameters.
//
// - BoxBOD: the Start 2 values (b1=100, b2=0.75) sit on a near-flat plateau of
//   the exponential model; plain MnMigrad does not reach the certified minimum
//   (b1=213.8, b2=0.547) from there.
//
// - MGH09: rational linear/quadratic model whose Start 2 point lies in a region
//   from which plain MnMigrad does not reach the certified minimum.
//
// - Hahn1: rational cubic/cubic model that requires x-axis rescaling to be well
//   conditioned (see `examples/nist_strd` `hahn_to_scaled`). Out of scope for a
//   plain-Start-2 oracle test.
