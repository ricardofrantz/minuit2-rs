use minuit2::{MnMigrad, MnScan};

/// 1D scan of a quadratic: should produce parabolic profile.
#[test]
fn scan_quadratic_profile() {
    let result = MnMigrad::new()
        .add("x", 5.0, 1.0)
        .add("y", -3.0, 1.0)
        .minimize(&|p: &[f64]| p[0] * p[0] + p[1] * p[1]);

    assert!(result.is_valid());

    let scan = MnScan::new(
        &|p: &[f64]| p[0] * p[0] + p[1] * p[1],
        &result,
    );
    let points = scan.scan(0, 20, -2.0, 2.0);

    assert!(!points.is_empty());
    assert!(points.len() >= 20);

    // Minimum should be near x=0
    let min_point = points.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
    assert!(
        min_point.0.abs() < 0.3,
        "minimum x should be near 0, got {}",
        min_point.0
    );
}

/// Auto-range scan: default is ±2*error.
#[test]
fn scan_auto_range() {
    let result = MnMigrad::new()
        .add("x", 0.0, 1.0)
        .minimize(&|p: &[f64]| p[0] * p[0]);

    assert!(result.is_valid());

    let scan = MnScan::new(&|p: &[f64]| p[0] * p[0], &result);
    // low == high == 0.0 triggers auto-range
    let points = scan.scan(0, 10, 0.0, 0.0);

    assert!(!points.is_empty());
    // Check range covers roughly ±2*error
    let x_min = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    let x_max = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
    assert!(x_min < -0.5, "auto-range should go below 0, got {x_min}");
    assert!(x_max > 0.5, "auto-range should go above 0, got {x_max}");
}

/// Scan finds better minimum.
#[test]
fn scan_minimum_tracking() {
    use minuit2::user_parameters::MnUserParameters;
    use minuit2::scan::MnParameterScan;

    // Start far from minimum
    let mut params = MnUserParameters::new();
    params.add("x", 10.0, 1.0);

    let fcn = |p: &[f64]| (p[0] - 3.0).powi(2);
    let initial_fval = fcn(&[10.0]);

    let mut scanner = MnParameterScan::new(&fcn, params, initial_fval);
    let _points = scanner.scan(0, 50, 0.0, 6.0);

    // Scanner should have found a better point near x=3
    assert!(
        scanner.fval() < initial_fval,
        "scanner should find better fval: {} < {}",
        scanner.fval(),
        initial_fval
    );
}
