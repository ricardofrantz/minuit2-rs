//! Rosenbrock function: Migrad + Hesse for accurate errors.
//!
//! Demonstrates the full minimization + error analysis workflow:
//! 1. Migrad finds the minimum
//! 2. Hesse computes the full Hessian for accurate parameter errors
//!
//! Run: cargo run --example rosenbrock_hesse

use minuit2::{MnHesse, MnMigrad};

fn main() {
    println!("=== Rosenbrock: Migrad + Hesse ===\n");

    let rosenbrock = |p: &[f64]| (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2);

    // Step 1: Migrad
    let result = MnMigrad::new()
        .add("x", 0.0, 0.1)
        .add("y", 0.0, 0.1)
        .minimize(&rosenbrock);

    println!("Migrad:");
    println!("  valid = {}", result.is_valid());
    println!("  nfcn  = {}", result.nfcn());
    println!("  fval  = {:.6e}", result.fval());
    println!("  edm   = {:.6e}", result.edm());

    let state = result.user_state();
    println!(
        "  x = {:.6} +/- {:.6}",
        state.value("x").unwrap(),
        state.error("x").unwrap()
    );
    println!(
        "  y = {:.6} +/- {:.6}",
        state.value("y").unwrap(),
        state.error("y").unwrap()
    );

    // Step 2: Hesse for accurate covariance
    let hesse = MnHesse::new().calculate(&rosenbrock, &result);

    println!("\nHesse:");
    println!("  valid = {}", hesse.is_valid());

    let hs = hesse.user_state();
    println!(
        "  x = {:.6} +/- {:.6}",
        hs.value("x").unwrap(),
        hs.error("x").unwrap()
    );
    println!(
        "  y = {:.6} +/- {:.6}",
        hs.value("y").unwrap(),
        hs.error("y").unwrap()
    );
    println!(
        "  covariance: {}",
        if hs.has_covariance() { "yes" } else { "no" }
    );

    if let Some(gcc) = hs.global_cc() {
        println!("  global correlations: [{:.4}, {:.4}]", gcc[0], gcc[1]);
    }

    // At (1,1): H = [[802, -400], [-400, 200]]
    // V = H^-1 → err_x ≈ 0.071, err_y ≈ 0.141
    println!("\nExpected: minimum at (1, 1), f = 0");
}
