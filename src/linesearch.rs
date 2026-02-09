//! Parabolic line search along a step direction.
//!
//! Replaces MnLineSearch.cxx. Performs a 1D parabolic interpolation to find
//! the optimal step size λ along a given search direction. Returns the
//! optimal (λ, f(λ)) pair.

use crate::minimum::parameters::MinimumParameters;
use crate::mn_fcn::MnFcn;
use crate::parabola::{MnParabolaPoint, from_3_points};
use crate::precision::MnMachinePrecision;
use nalgebra::DVector;

/// Perform a parabolic line search along `step` from `params`.
///
/// `gdel` is the directional derivative: step · gradient (should be negative
/// for a descent direction). Returns `MnParabolaPoint { x: λ_opt, y: f_opt }`.
pub fn mn_linesearch(
    fcn: &MnFcn,
    params: &MinimumParameters,
    step: &DVector<f64>,
    gdel: f64,
    prec: &MnMachinePrecision,
) -> MnParabolaPoint {
    let _overal = 1000.0;
    let undral = -100.0;
    let toler = 0.05;
    let slambg = 5.0;
    let alpha = 2.0;
    let maxiter = 12;

    let f0 = params.fval();
    let x0 = params.vec();

    // Evaluate at unit step
    let p1 = x0 + step;
    let f1 = fcn.call(p1.as_slice());

    let mut fvmin = f0;
    let mut xvmin = 0.0;
    if f1 < f0 {
        fvmin = f1;
        xvmin = 1.0;
    }

    let toler8 = toler;
    let slamax = slambg;
    let flast = f1;

    // Two-point parabolic interpolation using gradient at λ=0
    let mut slam = 1.0;
    let denom = 2.0 * (flast - f0 - gdel * slam) / (slam * slam);
    if denom.abs() > prec.eps2() {
        slam = -gdel / denom;
    } else {
        // Can't form parabola — try a large step
        slam = slamax;
    }

    // Clamp slam
    if slam > slamax {
        slam = slamax;
    }
    if slam < toler8 {
        slam = toler8;
    }
    if slam < 0.0 {
        slam = slamax;
    }

    // Check if first step was already good enough
    if (slam - 1.0).abs() < toler8 && f1 < f0 {
        return MnParabolaPoint::new(xvmin, fvmin);
    }

    // Evaluate at the interpolated step
    let p2 = x0 + slam * step;
    let f2 = fcn.call(p2.as_slice());

    if f2 < fvmin {
        fvmin = f2;
        xvmin = slam;
    }

    // Now we have 3 points: (0, f0), (1, f1), (slam, f2)
    // Set up for 3-point iteration
    let mut pt0 = MnParabolaPoint::new(0.0, f0);
    let mut pt1 = MnParabolaPoint::new(1.0, f1);
    let mut pt2 = MnParabolaPoint::new(slam, f2);

    // Sort so that pt0.x < pt1.x < pt2.x
    let mut pts = [pt0, pt1, pt2];
    pts.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    pt0 = pts[0];
    pt1 = pts[1];
    pt2 = pts[2];

    let slamax = (alpha * xvmin.abs()).max(slamax);

    for _niter in 0..maxiter {
        // Fit parabola through 3 points
        let pb = from_3_points(pt0, pt1, pt2);

        // Only use parabola minimum if curvature is positive (upward parabola)
        if pb.a() < prec.eps2() {
            // Flat or downward — can't trust the minimum, break
            break;
        }

        slam = pb.min();

        // Clamp slam to reasonable range
        if slam > slamax {
            slam = slamax;
        }
        if slam < undral {
            slam = undral;
        }
        if slam < 0.0 && pb.y(0.0) < pb.y(slam) {
            // Going backward makes things worse — stop
            break;
        }

        // Don't evaluate too close to existing points
        let toler9 = toler * slam.abs().max(1.0);
        if (slam - pt0.x).abs() < toler9
            || (slam - pt1.x).abs() < toler9
            || (slam - pt2.x).abs() < toler9
        {
            break;
        }

        // Evaluate at new slam
        let p_new = x0 + slam * step;
        let f_new = fcn.call(p_new.as_slice());

        if f_new < fvmin {
            fvmin = f_new;
            xvmin = slam;
        }

        // Replace the worst of the 3 points
        let new_pt = MnParabolaPoint::new(slam, f_new);

        // Find which existing point has the highest f value
        if pt0.y > pt1.y && pt0.y > pt2.y {
            pt0 = new_pt;
        } else if pt2.y > pt1.y {
            pt2 = new_pt;
        } else {
            pt1 = new_pt;
        }

        // Re-sort
        let mut pts = [pt0, pt1, pt2];
        pts.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        pt0 = pts[0];
        pt1 = pts[1];
        pt2 = pts[2];

        // Check convergence: improvement is tiny
        if (fvmin - f0).abs() < f0.abs() * prec.eps() {
            break;
        }
    }

    MnParabolaPoint::new(xvmin, fvmin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fcn::FCN;
    use crate::user_transformation::MnUserTransformation;
    use crate::parameter::MinuitParameter;

    struct Quadratic;
    impl FCN for Quadratic {
        fn value(&self, p: &[f64]) -> f64 {
            p[0] * p[0]
        }
    }

    #[test]
    fn linesearch_quadratic() {
        let params = vec![MinuitParameter::new(0, "x", 2.0, 0.1)];
        let trafo = MnUserTransformation::new(params);
        let fcn = MnFcn::new(&Quadratic, &trafo);

        // Start at x=2, step direction = -1 (downhill)
        let start = MinimumParameters::new(DVector::from_vec(vec![2.0]), 4.0);
        let step = DVector::from_vec(vec![-1.0]);
        let gdel = step.dot(&DVector::from_vec(vec![4.0])); // grad = 2x = 4 at x=2
        let prec = MnMachinePrecision::new();

        let result = mn_linesearch(&fcn, &start, &step, gdel, &prec);

        // Optimal step for f(2-λ) = (2-λ)² is λ=2, giving f=0
        assert!(result.y < 4.0, "line search should improve: f={}", result.y);
        assert!(result.y < 0.1, "line search should find near-minimum: f={}", result.y);
    }
}
