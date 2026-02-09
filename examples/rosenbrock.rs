//! Rosenbrock function minimization with Simplex.
//!
//! Run: cargo run --example rosenbrock

use minuit2::MnSimplex;

fn main() {
    println!("=== Rosenbrock 2D Minimization with Simplex ===\n");

    let result = MnSimplex::new()
        .add("x", -1.0, 1.0)
        .add("y", -1.0, 1.0)
        .max_fcn(10000)
        .minimize(&|p: &[f64]| {
            (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2)
        });

    println!("{result}");

    if result.is_valid() {
        println!("Minimization converged successfully.");
    } else {
        println!("WARNING: minimization did not converge.");
        if result.reached_call_limit() {
            println!("  Reached function call limit.");
        }
        if result.is_above_max_edm() {
            println!("  EDM above maximum.");
        }
    }
}
