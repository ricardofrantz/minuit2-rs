/// Function to be minimized.
///
/// The core trait that users implement. `error_def()` returns the change in FCN
/// that corresponds to one standard deviation (1.0 for chi-square, 0.5 for
/// log-likelihood).
pub trait FCN {
    fn value(&self, par: &[f64]) -> f64;

    /// FCN change corresponding to 1-sigma. Default = 1.0 (chi-square).
    fn error_def(&self) -> f64 {
        1.0
    }
}

/// Blanket impl: any `Fn(&[f64]) -> f64` is a valid FCN with error_def = 1.0.
impl<F> FCN for F
where
    F: Fn(&[f64]) -> f64,
{
    fn value(&self, par: &[f64]) -> f64 {
        self(par)
    }
}

/// FCN that also provides analytical gradients.
pub trait FCNGradient: FCN {
    fn gradient(&self, par: &[f64]) -> Vec<f64>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closure_as_fcn() {
        let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
        assert!((FCN::value(&f, &[3.0, 4.0]) - 25.0).abs() < 1e-15);
        assert!((f.error_def() - 1.0).abs() < 1e-15);
    }

    #[test]
    fn struct_fcn() {
        struct Rosenbrock;
        impl FCN for Rosenbrock {
            fn value(&self, p: &[f64]) -> f64 {
                (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2)
            }
            fn error_def(&self) -> f64 {
                0.5
            }
        }
        let f = Rosenbrock;
        assert!((f.value(&[1.0, 1.0])).abs() < 1e-15);
        assert!((f.error_def() - 0.5).abs() < 1e-15);
    }
}
