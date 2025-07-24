use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_steam::iapws97;

pub fn criterion_benchmark(c: &mut Criterion) {
    // Region 5 hmass_tp
    c.bench_function("Region 5 hmass_tp-1", |b| {
        b.iter(|| iapws97::hmass_tp(black_box(1500.0), black_box(0.5e6)))
    });

    c.bench_function("Region 5 hmass_tp-2", |b| {
        b.iter(|| iapws97::hmass_tp(black_box(1500.0), black_box(30e6)))
    });

    c.bench_function("Region 5 hmass_tp-3", |b| {
        b.iter(|| iapws97::hmass_tp(black_box(2000.0), black_box(30e6)))
    });

    // Region 5 umass_tp
    c.bench_function("Region 5 umass_tp-1", |b| {
        b.iter(|| iapws97::umass_tp(black_box(1500.0), black_box(0.5e6)))
    });

    c.bench_function("Region 5 umass_tp-2", |b| {
        b.iter(|| iapws97::umass_tp(black_box(1500.0), black_box(30e6)))
    });

    c.bench_function("Region 5 umass_tp-3", |b| {
        b.iter(|| iapws97::umass_tp(black_box(2000.0), black_box(30e6)))
    });

    // Region 5 smass_tp
    c.bench_function("Region 5 smass_tp-1", |b| {
        b.iter(|| iapws97::smass_tp(black_box(1500.0), black_box(0.5e6)))
    });

    c.bench_function("Region 5 smass_tp-2", |b| {
        b.iter(|| iapws97::smass_tp(black_box(1500.0), black_box(30e6)))
    });

    c.bench_function("Region 5 smass_tp-3", |b| {
        b.iter(|| iapws97::smass_tp(black_box(2000.0), black_box(30e6)))
    });

    // Region 5 cpmass_tp
    c.bench_function("Region 5 cpmass_tp-1", |b| {
        b.iter(|| iapws97::cpmass_tp(black_box(1500.0), black_box(0.5e6)))
    });

    c.bench_function("Region 5 cpmass_tp-2", |b| {
        b.iter(|| iapws97::cpmass_tp(black_box(1500.0), black_box(30e6)))
    });

    c.bench_function("Region 5 cpmass_tp-3", |b| {
        b.iter(|| iapws97::cpmass_tp(black_box(2000.0), black_box(30e6)))
    });

    // Region 5 cvmass_tp
    c.bench_function("Region 5 cvmass_tp-1", |b| {
        b.iter(|| iapws97::cvmass_tp(black_box(1500.0), black_box(0.5e6)))
    });

    c.bench_function("Region 5 cvmass_tp-2", |b| {
        b.iter(|| iapws97::cvmass_tp(black_box(1500.0), black_box(30e6)))
    });

    c.bench_function("Region 5 cvmass_tp-3", |b| {
        b.iter(|| iapws97::cvmass_tp(black_box(2000.0), black_box(30e6)))
    });

    // Region 5 vmass_tp
    c.bench_function("Region 5 vmass_tp-1", |b| {
        b.iter(|| iapws97::vmass_tp(black_box(1500.0), black_box(0.5e6)))
    });

    c.bench_function("Region 5 vmass_tp-2", |b| {
        b.iter(|| iapws97::vmass_tp(black_box(1500.0), black_box(30e6)))
    });

    c.bench_function("Region 5 vmass_tp-3", |b| {
        b.iter(|| iapws97::vmass_tp(black_box(2000.0), black_box(30e6)))
    });

    // Region 5 speed_sound_tp
    c.bench_function("Region 5 speed_sound_tp-1", |b| {
        b.iter(|| iapws97::speed_sound_tp(black_box(1500.0), black_box(0.5e6)))
    });

    c.bench_function("Region 5 speed_sound_tp-2", |b| {
        b.iter(|| iapws97::speed_sound_tp(black_box(1500.0), black_box(30e6)))
    });

    c.bench_function("Region 5 speed_sound_tp-3", |b| {
        b.iter(|| iapws97::speed_sound_tp(black_box(2000.0), black_box(30e6)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
