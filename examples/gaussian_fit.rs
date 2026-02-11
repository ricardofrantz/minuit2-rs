//! Gaussian fit with Migrad + Hesse + Minos.
//!
//! Fits a Gaussian model y = A * exp(-(x-mu)^2 / (2*sigma^2)) to
//! synthetic data using chi-square minimization, then computes
//! parabolic (Hesse) and asymmetric (Minos) parameter errors.
//!
//! Run: cargo run --example gaussian_fit

use minuit2::{FCN, MnHesse, MnMigrad, MnMinos};

/// Chi-square FCN for Gaussian model fit.
struct GaussianChi2 {
    x: Vec<f64>,
    y: Vec<f64>,
    sigma: Vec<f64>,
}

impl FCN for GaussianChi2 {
    fn value(&self, p: &[f64]) -> f64 {
        let amp = p[0];
        let mu = p[1];
        let sig = p[2];
        self.x
            .iter()
            .zip(self.y.iter())
            .zip(self.sigma.iter())
            .map(|((&xi, &yi), &si)| {
                let model = amp * (-0.5 * ((xi - mu) / sig).powi(2)).exp();
                ((yi - model) / si).powi(2)
            })
            .sum()
    }

    fn error_def(&self) -> f64 {
        1.0 // chi-square
    }
}

fn main() {
    println!("=== Gaussian Fit: Migrad + Hesse + Minos ===\n");

    // Synthetic data: A=10, mu=5, sigma=1.5, with known measurement errors
    let x: Vec<f64> = (0..21).map(|i| i as f64 * 0.5).collect();
    let y_true: Vec<f64> = x
        .iter()
        .map(|&xi| 10.0 * (-0.5 * ((xi - 5.0) / 1.5).powi(2)).exp())
        .collect();
    // Add deterministic "noise" (small sinusoidal perturbation)
    let y: Vec<f64> = y_true
        .iter()
        .enumerate()
        .map(|(i, &yt)| yt + 0.3 * (i as f64 * 1.7).sin())
        .collect();
    let sigma: Vec<f64> = vec![0.5; y.len()];

    let fcn = GaussianChi2 {
        x: x.clone(),
        y,
        sigma,
    };

    println!("Data: {} points, true A=10, mu=5, sigma=1.5\n", x.len());

    // Step 1: Migrad
    let result = MnMigrad::new()
        .add("A", 8.0, 1.0)
        .add("mu", 4.0, 0.5)
        .add_lower_limited("sigma", 2.0, 0.5, 0.01)
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

    println!("\nHesse errors:");
    for name in &["A", "mu", "sigma"] {
        println!(
            "  {} = {:.4} +/- {:.4}",
            name,
            hs.value(name).unwrap(),
            hs.error(name).unwrap()
        );
    }

    // Step 3: Minos
    let minos = MnMinos::new(&fcn, &hesse);

    println!("\nMinos errors:");
    for (i, name) in ["A", "mu", "sigma"].iter().enumerate() {
        let me = minos.minos_error(i);
        if me.is_valid() {
            println!(
                "  {} = {:.4}  {:.4} / +{:.4}",
                name,
                hs.value(name).unwrap(),
                me.lower_error(),
                me.upper_error()
            );
        } else {
            println!("  {}: Minos did not converge", name);
        }
    }
}
