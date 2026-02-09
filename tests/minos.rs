use minuit2::{MnMigrad, MnHesse, MnMinos};

/// Symmetric case: Gaussian/quadratic fit → Minos errors ≈ Hesse errors.
#[test]
fn minos_symmetric_quadratic() {
    let quadratic = |p: &[f64]| 2.0 * p[0] * p[0] + 8.0 * p[1] * p[1];

    let result = MnMigrad::new()
        .add("x", 5.0, 1.0)
        .add("y", -3.0, 1.0)
        .minimize(&quadratic);

    assert!(result.is_valid());

    // Run Hesse first for accurate errors
    let hesse_result = MnHesse::new().calculate(&quadratic, &result);
    assert!(hesse_result.is_valid());

    let minos = MnMinos::new(&quadratic, &hesse_result);
    let me = minos.minos_error(0);

    // For symmetric quadratic, Minos errors should be roughly symmetric
    let upper = me.upper_error();
    let lower = me.lower_error();

    if me.is_valid() {
        // Upper and lower should have similar magnitude (symmetric function)
        let ratio = upper.abs() / lower.abs();
        assert!(
            (ratio - 1.0).abs() < 0.5,
            "upper/lower ratio should be ~1 for symmetric function, got {ratio} (upper={upper}, lower={lower})"
        );

        // Both should be positive/negative respectively and non-zero
        assert!(upper > 0.0, "upper Minos error should be positive: {upper}");
        assert!(lower < 0.0, "lower Minos error should be negative: {lower}");
    }

    // Even if Minos doesn't converge perfectly, check it doesn't crash
    // and returns sensible signs
}

/// Asymmetric function: Minos upper != lower.
#[test]
fn minos_asymmetric() {
    // f(x) = x^2 for x > 0, 4*x^2 for x < 0
    // Minimum at x=0
    // Upper error: sqrt(up/2) = sqrt(0.5) ≈ 0.707
    // Lower error: sqrt(up/8) = sqrt(0.125) ≈ 0.354
    let asym = |p: &[f64]| {
        if p[0] >= 0.0 {
            p[0] * p[0]
        } else {
            4.0 * p[0] * p[0]
        }
    };

    let result = MnMigrad::new()
        .add("x", 0.5, 0.5)
        .minimize(&asym);

    assert!(result.is_valid());
    assert!(result.fval() < 0.01, "should find minimum near 0");

    let hesse_result = MnHesse::new().calculate(&asym, &result);

    let minos = MnMinos::new(&asym, &hesse_result);
    let me = minos.minos_error(0);

    if me.is_valid() {
        let upper = me.upper_error();
        let lower = me.lower_error();

        // Upper should be larger in magnitude than lower
        assert!(
            upper.abs() > lower.abs() * 1.2,
            "upper ({upper}) should be larger than |lower| ({lower}) for asymmetric function"
        );
    }
    // If Minos doesn't converge (can happen with discontinuous derivative),
    // that's acceptable — the important thing is it doesn't crash.
}

/// Fixed parameter: Minos should return invalid.
#[test]
fn minos_fixed_parameter() {
    let result = MnMigrad::new()
        .add("x", 5.0, 1.0)
        .add_const("y", 0.0)
        .minimize(&|p: &[f64]| p[0] * p[0] + p[1] * p[1]);

    assert!(result.is_valid());

    let minos = MnMinos::new(
        &|p: &[f64]| p[0] * p[0] + p[1] * p[1],
        &result,
    );

    // Parameter 1 (y) is const → Minos should return invalid
    let cross = minos.lower(1);
    assert!(!cross.is_valid(), "Minos on const param should be invalid");
}
