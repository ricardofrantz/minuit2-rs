//! Migrad minimization of the Rosenbrock function.
//!
//! Demonstrates the MnMigrad quasi-Newton minimizer on the classic
//! Rosenbrock "banana valley" function: f(x,y) = (1-x)² + 100(y-x²)².
//! The minimum is at (1, 1) with f = 0.

use minuit2::MnMigrad;

fn main() {
    let result = MnMigrad::new()
        .add("x", -1.0, 1.0)
        .add("y", -1.0, 1.0)
        .minimize(&|p: &[f64]| (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2));

    println!("{result}");
    println!();

    if result.is_valid() {
        println!("Migrad converged in {} function calls", result.nfcn());
        let params = result.params();
        println!("Minimum at: x = {:.6}, y = {:.6}", params[0], params[1]);
        println!("Function value: {:.6e}", result.fval());
        println!("EDM: {:.6e}", result.edm());
    } else {
        println!("WARNING: Migrad did not converge!");
        if result.reached_call_limit() {
            println!("  Reason: reached function call limit");
        }
        if result.is_above_max_edm() {
            println!("  Reason: EDM above maximum threshold");
        }
    }
}
