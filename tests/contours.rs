use minuit2::{MnMigrad, MnHesse, MnContours};

/// 2D quadratic: contour points should form approximate ellipse.
#[test]
fn contour_quadratic_ellipse() {
    let quadratic = |p: &[f64]| 2.0 * p[0] * p[0] + 8.0 * p[1] * p[1];

    let result = MnMigrad::new()
        .add("x", 5.0, 1.0)
        .add("y", -3.0, 1.0)
        .minimize(&quadratic);

    assert!(result.is_valid());

    let hesse_result = MnHesse::new().calculate(&quadratic, &result);

    let contours = MnContours::new(&quadratic, &hesse_result);
    let points = contours.points(0, 1, 8);

    // Should have at least 4 points (the cardinal MINOS points)
    assert!(
        points.len() >= 4,
        "contour should have >= 4 points, got {}",
        points.len()
    );

    // All points should be at approximately the same function value: fmin + up
    let up = hesse_result.up();
    let fmin = hesse_result.fval();
    let target = fmin + up;

    for (x, y) in &points {
        let f = quadratic(&[*x, *y]);
        // Contour points should be approximately at F = fmin + up
        // Allow generous tolerance since contour fitting is approximate
        assert!(
            (f - target).abs() < 0.5 * up,
            "contour point ({x}, {y}) has f={f}, expected ~{target}"
        );
    }
}

/// Correlated 2D quadratic: contour should be tilted ellipse.
#[test]
fn contour_correlated() {
    // f(x,y) = x^2 + y^2 + x*y
    let correlated = |p: &[f64]| p[0] * p[0] + p[1] * p[1] + p[0] * p[1];

    let result = MnMigrad::new()
        .add("x", 5.0, 1.0)
        .add("y", -3.0, 1.0)
        .minimize(&correlated);

    assert!(result.is_valid());

    let hesse_result = MnHesse::new().calculate(&correlated, &result);

    let contours = MnContours::new(&correlated, &hesse_result);
    let points = contours.points(0, 1, 8);

    assert!(
        points.len() >= 4,
        "contour should have >= 4 points, got {}",
        points.len()
    );

    // Points should not all lie on axis-aligned ellipse (correlation tilts it)
    // Check that at least one point has both x and y non-zero
    let off_axis = points.iter().any(|(x, y)| x.abs() > 0.01 && y.abs() > 0.01);
    assert!(off_axis, "correlated contour should have off-axis points");
}
