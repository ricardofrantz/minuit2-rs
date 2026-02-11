//! Quadratic polynomial fit with chi-square and parameter scan.
//!
//! Fits y = c0 + c1*x + c2*x^2 to synthetic data with known uncertainties,
//! then scans one parameter to visualize the chi-square profile.
//!
//! Run: cargo run --example chi_square

use minuit2::{FCN, MnHesse, MnMigrad, MnScan};

/// Chi-square FCN for quadratic polynomial.
struct PolyChi2 {
    x: Vec<f64>,
    y: Vec<f64>,
    sigma: Vec<f64>,
}

impl FCN for PolyChi2 {
    fn value(&self, p: &[f64]) -> f64 {
        self.x
            .iter()
            .zip(self.y.iter())
            .zip(self.sigma.iter())
            .map(|((&xi, &yi), &si)| {
                let model = p[0] + p[1] * xi + p[2] * xi * xi;
                ((yi - model) / si).powi(2)
            })
            .sum()
    }

    fn error_def(&self) -> f64 {
        1.0
    }
}

fn main() {
    println!("=== Quadratic Fit with Chi-Square + Scan ===\n");

    // True model: y = 2.0 + 0.5*x + 0.1*x^2
    let x: Vec<f64> = (0..15).map(|i| i as f64).collect();
    let y_true: Vec<f64> = x.iter().map(|&xi| 2.0 + 0.5 * xi + 0.1 * xi * xi).collect();
    // Deterministic perturbation
    let y: Vec<f64> = y_true
        .iter()
        .enumerate()
        .map(|(i, &yt)| yt + 0.2 * ((i as f64) * 2.3).sin())
        .collect();
    // 5% relative error, minimum 0.3
    let sigma: Vec<f64> = y_true
        .iter()
        .map(|&yt| (0.05 * yt.abs()).max(0.3))
        .collect();

    let fcn = PolyChi2 {
        x: x.clone(),
        y,
        sigma,
    };

    println!("Data: {} points, true c0=2.0, c1=0.5, c2=0.1\n", x.len());

    // Step 1: Migrad
    let result = MnMigrad::new()
        .add("c0", 1.0, 0.1)
        .add("c1", 0.3, 0.05)
        .add("c2", 0.05, 0.01)
        .minimize(&fcn);

    let ndf = x.len() as f64 - 3.0;
    println!(
        "Migrad: valid={}, chi2={:.2}, ndf={:.0}, chi2/ndf={:.2}",
        result.is_valid(),
        result.fval(),
        ndf,
        result.fval() / ndf
    );

    // Step 2: Hesse
    let hesse = MnHesse::new().calculate(&fcn, &result);
    let hs = hesse.user_state();

    println!("\nFitted parameters:");
    for name in &["c0", "c1", "c2"] {
        println!(
            "  {} = {:.6} +/- {:.6}",
            name,
            hs.value(name).unwrap(),
            hs.error(name).unwrap()
        );
    }

    // Step 3: Scan c2
    let scan = MnScan::new(&fcn, &hesse);
    let points = scan.scan(2, 20, 0.0, 0.0); // auto-range

    println!("\nScan of c2 (auto-range +/- 2*sigma):");
    println!("  {:>10} {:>12}", "c2", "chi2");
    for (c2_val, chi2_val) in &points {
        println!("  {:10.6} {:12.4}", c2_val, chi2_val);
    }

    let min_pt = points
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();
    println!("\n  Scan minimum: c2={:.6}, chi2={:.4}", min_pt.0, min_pt.1);
}
