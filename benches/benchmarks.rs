use criterion::{criterion_group, criterion_main, Criterion, black_box};
use minuit2::{MnMigrad, MnSimplex, MnHesse, FCN};

fn bench_rosenbrock_migrad(c: &mut Criterion) {
    c.bench_function("Rosenbrock 2D: MnMigrad minimize", |b| {
        b.iter(|| {
            let rosenbrock = |p: &[f64]| {
                (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2)
            };
            let result = MnMigrad::new()
                .add("x", 0.0, 0.1)
                .add("y", 0.0, 0.1)
                .minimize(&rosenbrock);
            black_box(result);
        })
    });
}

fn bench_rosenbrock_simplex(c: &mut Criterion) {
    c.bench_function("Rosenbrock 2D: MnSimplex minimize", |b| {
        b.iter(|| {
            let rosenbrock = |p: &[f64]| {
                (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2)
            };
            let result = MnSimplex::new()
                .add("x", 0.0, 0.1)
                .add("y", 0.0, 0.1)
                .minimize(&rosenbrock);
            black_box(result);
        })
    });
}

fn bench_quadratic_4d_migrad(c: &mut Criterion) {
    c.bench_function("Quadratic 4D: MnMigrad minimize", |b| {
        b.iter(|| {
            let quadratic = |p: &[f64]| {
                p[0] * p[0] + p[1] * p[1] + p[2] * p[2] + p[3] * p[3]
            };
            let result = MnMigrad::new()
                .add("x1", 1.0, 0.1)
                .add("x2", 2.0, 0.1)
                .add("x3", 3.0, 0.1)
                .add("x4", 4.0, 0.1)
                .minimize(&quadratic);
            black_box(result);
        })
    });
}

fn bench_quadratic_2d_migrad_hesse(c: &mut Criterion) {
    c.bench_function("Quadratic 2D: MnMigrad + MnHesse", |b| {
        b.iter(|| {
            let quadratic = |p: &[f64]| p[0] * p[0] + p[1] * p[1];
            let migrad_result = MnMigrad::new()
                .add("x", 1.0, 0.1)
                .add("y", 2.0, 0.1)
                .minimize(&quadratic);
            let hesse_result = MnHesse::new().calculate(&quadratic, &migrad_result);
            black_box((migrad_result, hesse_result));
        })
    });
}

struct GaussianChi2 {
    x: Vec<f64>,
    y: Vec<f64>,
}

impl FCN for GaussianChi2 {
    fn value(&self, p: &[f64]) -> f64 {
        let (a, mu, sigma) = (p[0], p[1], p[2]);
        self.x.iter().zip(self.y.iter()).map(|(&xi, &yi)| {
            let model = a * (-0.5 * ((xi - mu) / sigma).powi(2)).exp();
            (yi - model).powi(2)
        }).sum()
    }
}

fn bench_gaussian_fit_migrad_hesse(c: &mut Criterion) {
    c.bench_function("Gaussian fit (chi-square, 3 params): Migrad+Hesse pipeline", |b| {
        b.iter(|| {
            let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let y = vec![0.1, 2.3, 7.8, 6.1, 0.2];
            let fcn = GaussianChi2 { x, y };
            let migrad_result = MnMigrad::new()
                .add("A", 8.0, 1.0)
                .add("mu", 4.0, 0.5)
                .add_lower_limited("sigma", 2.0, 0.5, 0.01)
                .minimize(&fcn);
            let hesse_result = MnHesse::new().calculate(&fcn, &migrad_result);
            black_box((migrad_result, hesse_result));
        })
    });
}

criterion_group!(
    benches,
    bench_rosenbrock_migrad,
    bench_rosenbrock_simplex,
    bench_quadratic_4d_migrad,
    bench_quadratic_2d_migrad_hesse,
    bench_gaussian_fit_migrad_hesse
);
criterion_main!(benches);
