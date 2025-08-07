use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_steam::iapws97;

pub fn criterion_benchmark(c: &mut Criterion) {
    if cfg!(feature = "nightly") {
        // Region 1 hmass_tp
        c.bench_function("SIMD Region 1 hmass_tp-1", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("SIMD Region 1 hmass_tp-2", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("SIMD Region 1 hmass_tp-3", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 umass_tp
        c.bench_function("SIMD Region 1 umass_tp-1", |b| {
            b.iter(|| iapws97::umass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("SIMD Region 1 umass_tp-2", |b| {
            b.iter(|| iapws97::umass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("SIMD Region 1 umass_tp-3", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 smass_tp
        c.bench_function("SIMD Region 1 smass_tp-1", |b| {
            b.iter(|| iapws97::smass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("SIMD Region 1 smass_tp-2", |b| {
            b.iter(|| iapws97::smass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("SIMD Region 1 smass_tp-3", |b| {
            b.iter(|| iapws97::smass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 cpmass_tp
        c.bench_function("SIMD Region 1 cpmass_tp-1", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("SIMD Region 1 cpmass_tp-2", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("SIMD Region 1 cpmass_tp-3", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 cvmass_tp
        c.bench_function("SIMD Region 1 cvmass_tp-1", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("SIMD Region 1 cvmass_tp-2", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("SIMD Region 1 cvmass_tp-3", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 vmass_tp
        c.bench_function("SIMD Region 1 vmass_tp-1", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("SIMD Region 1 vmass_tp-2", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("SIMD Region 1 vmass_tp-3", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 speed_sound_tp
        c.bench_function("SIMD Region 1 speed_sound_tp-1", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("SIMD Region 1 speed_sound_tp-2", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("SIMD Region 1 speed_sound_tp-3", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 temperature_ps
        c.bench_function("SIMD Region 1 temperature_ps-1", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(3e6), black_box(0.5)))
        });

        c.bench_function("SIMD Region 1 temperature_ps-2", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(80e6), black_box(0.5)))
        });

        c.bench_function("SIMD Region 1 temperature_ps-3", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(80e6), black_box(3.0)))
        });

        // Region 1 temperature_ph
        c.bench_function("SIMD Region 1 temperature_ph-1", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(3e6), black_box(500.0)))
        });

        c.bench_function("SIMD Region 1 temperature_ph-2", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(80e6), black_box(500.0)))
        });

        c.bench_function("SIMD Region 1 temperature_ph-3", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(80e6), black_box(1500.0)))
        });
    } else {
        // Region 1 hmass_tp
        c.bench_function("Region 1 hmass_tp-1", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("Region 1 hmass_tp-2", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("Region 1 hmass_tp-3", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 umass_tp
        c.bench_function("Region 1 umass_tp-1", |b| {
            b.iter(|| iapws97::umass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("Region 1 umass_tp-2", |b| {
            b.iter(|| iapws97::umass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("Region 1 umass_tp-3", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 smass_tp
        c.bench_function("Region 1 smass_tp-1", |b| {
            b.iter(|| iapws97::smass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("Region 1 smass_tp-2", |b| {
            b.iter(|| iapws97::smass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("Region 1 smass_tp-3", |b| {
            b.iter(|| iapws97::smass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 cpmass_tp
        c.bench_function("Region 1 cpmass_tp-1", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("Region 1 cpmass_tp-2", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("Region 1 cpmass_tp-3", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 cvmass_tp
        c.bench_function("Region 1 cvmass_tp-1", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("Region 1 cvmass_tp-2", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("Region 1 cvmass_tp-3", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 vmass_tp
        c.bench_function("Region 1 vmass_tp-1", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("Region 1 vmass_tp-2", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("Region 1 vmass_tp-3", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 speed_sound_tp
        c.bench_function("Region 1 speed_sound_tp-1", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(300.0), black_box(3e6)))
        });

        c.bench_function("Region 1 speed_sound_tp-2", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(300.0), black_box(80e6)))
        });

        c.bench_function("Region 1 speed_sound_tp-3", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(500.0), black_box(3e6)))
        });

        // Region 1 temperature_ps
        c.bench_function("Region 1 temperature_ps-1", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(3e6), black_box(0.5)))
        });

        c.bench_function("Region 1 temperature_ps-2", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(80e6), black_box(0.5)))
        });

        c.bench_function("Region 1 temperature_ps-3", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(80e6), black_box(3.0)))
        });

        // Region 1 temperature_ph
        c.bench_function("Region 1 temperature_ph-1", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(3e6), black_box(500.0)))
        });

        c.bench_function("Region 1 temperature_ph-2", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(80e6), black_box(500.0)))
        });

        c.bench_function("Region 1 temperature_ph-3", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(80e6), black_box(1500.0)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
