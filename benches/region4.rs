use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_steam::iapws97;

pub fn criterion_benchmark(c: &mut Criterion) {
    // Saturation Pressure
    c.bench_function("psat97-1", |b| {
        b.iter(|| iapws97::psat97(black_box(&300.0)))
    });

    c.bench_function("psat97-2", |b| {
        b.iter(|| iapws97::psat97(black_box(&500.0)))
    });

    c.bench_function("psat97-3", |b| {
        b.iter(|| iapws97::psat97(black_box(&600.0)))
    });

    // Saturation Temperature
    c.bench_function("tsat97-1", |b| {
        b.iter(|| iapws97::tsat97(black_box(&0.1e6)))
    });

    c.bench_function("tsat97-2", |b| b.iter(|| iapws97::tsat97(black_box(&1e6))));

    c.bench_function("tsat97-3", |b| b.iter(|| iapws97::tsat97(black_box(&10e6))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
