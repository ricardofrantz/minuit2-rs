use std::f64::consts::FRAC_PI_2;

use crate::precision::MnMachinePrecision;
use super::ParameterTransform;

/// Sine transform for doubly-bounded parameters.
///
/// Maps \[lower, upper\] ↔ (-∞, +∞) using arcsin/sin.
/// Exact formulas from SinParameterTransformation.cxx.
pub struct SinTransform;

impl ParameterTransform for SinTransform {
    fn int2ext(&self, value: f64, upper: f64, lower: f64) -> f64 {
        lower + 0.5 * (upper - lower) * (value.sin() + 1.0)
    }

    fn ext2int(&self, value: f64, upper: f64, lower: f64, prec: &MnMachinePrecision) -> f64 {
        let piby2 = FRAC_PI_2;
        let distnn = 8.0 * (prec.eps2()).sqrt();
        let vlimhi = piby2 - distnn;
        let vlimlo = -piby2 + distnn;

        let yy = 2.0 * (value - lower) / (upper - lower) - 1.0;
        let yy2 = yy.abs();

        if yy2 >= 1.0 - distnn {
            // At boundary — clamp to avoid numerical issues
            if yy < 0.0 { vlimlo } else { vlimhi }
        } else {
            yy.asin()
        }
    }

    fn dint2ext(&self, value: f64, upper: f64, lower: f64) -> f64 {
        0.5 * ((upper - lower) * value.cos()).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let t = SinTransform;
        let prec = MnMachinePrecision::new();
        let (lo, hi) = (1.0, 10.0);

        for &ext in &[2.0, 5.5, 9.0] {
            let int = t.ext2int(ext, hi, lo, &prec);
            let back = t.int2ext(int, hi, lo);
            assert!((back - ext).abs() < 1e-12, "roundtrip failed for {ext}");
        }
    }

    #[test]
    fn midpoint() {
        let t = SinTransform;
        // internal = 0 should map to midpoint
        let ext = t.int2ext(0.0, 10.0, 0.0);
        assert!((ext - 5.0).abs() < 1e-15);
    }

    #[test]
    fn derivative_positive() {
        let t = SinTransform;
        let d = t.dint2ext(0.0, 10.0, 0.0);
        assert!(d > 0.0);
    }

    #[test]
    fn near_boundary() {
        let t = SinTransform;
        let prec = MnMachinePrecision::new();
        // Very close to upper bound
        let int = t.ext2int(9.9999999999, 10.0, 0.0, &prec);
        let ext = t.int2ext(int, 10.0, 0.0);
        assert!((ext - 10.0).abs() < 0.01);
    }
}
