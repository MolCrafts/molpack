//! Regression microbench for restraint evaluation.
//!
//! The pair kernel calls each atom's restraint through `AtomRestraint::f`
//! (function value) and `AtomRestraint::fg` (value + accumulated gradient) on
//! every evaluate. This bench times those two methods on the two most common
//! built-in restraints (`InsideBoxRestraint`, `InsideSphereRestraint`) over a
//! small batch of synthesized points that straddle the boundary (some inside
//! → zero penalty, some outside → active quadratic penalty), so both the
//! short-circuit and the penalty branches are exercised.
//!
//! Small and self-contained (128 points, pure functions): no external test
//! data, no features.

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use molpack::{AtomRestraint, F, InsideBoxRestraint, InsideSphereRestraint};

/// Small point batch: enough to amortize call overhead, still << 1 s total.
const N_POINTS: usize = 128;

/// Synthesize points in a cube slightly larger than the restraint region so a
/// mix fall inside (zero penalty) and outside (active penalty).
fn build_points(seed: u64) -> Vec<[F; 3]> {
    let mut rng = SmallRng::seed_from_u64(seed);
    (0..N_POINTS)
        .map(|_| {
            [
                rng.random::<F>() * 24.0 - 2.0,
                rng.random::<F>() * 24.0 - 2.0,
                rng.random::<F>() * 24.0 - 2.0,
            ]
        })
        .collect()
}

fn bench_restraint_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("restraint_eval");
    group.sample_size(10);
    group.measurement_time(Duration::from_millis(500));

    let points = build_points(0x5eed);
    // Packmol scale conventions: scale = 1.0, scale2 = 0.01.
    let (scale, scale2): (F, F) = (1.0, 0.01);

    let box_r = InsideBoxRestraint::new([0.0; 3], [20.0; 3], [false; 3]);
    let sphere_r = InsideSphereRestraint::new([10.0; 3], 10.0);

    group.bench_function("inside_box/f", |b| {
        b.iter(|| {
            let mut acc = 0.0;
            for p in &points {
                acc += box_r.f(p, scale, scale2);
            }
            std::hint::black_box(acc);
        });
    });

    group.bench_function("inside_box/fg", |b| {
        b.iter(|| {
            let mut acc = 0.0;
            let mut g = [0.0; 3];
            for p in &points {
                acc += box_r.fg(p, scale, scale2, &mut g);
            }
            std::hint::black_box((acc, g));
        });
    });

    group.bench_function("inside_sphere/f", |b| {
        b.iter(|| {
            let mut acc = 0.0;
            for p in &points {
                acc += sphere_r.f(p, scale, scale2);
            }
            std::hint::black_box(acc);
        });
    });

    group.bench_function("inside_sphere/fg", |b| {
        b.iter(|| {
            let mut acc = 0.0;
            let mut g = [0.0; 3];
            for p in &points {
                acc += sphere_r.fg(p, scale, scale2, &mut g);
            }
            std::hint::black_box((acc, g));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_restraint_eval);
criterion_main!(benches);
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::SmallRng;
