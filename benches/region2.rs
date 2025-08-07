use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_steam::iapws97;

pub fn criterion_benchmark(c: &mut Criterion) {
    if cfg!(feature = "nightly") {
        // Region 2 hmass_tp
        c.bench_function("SIMD Region 2 hmass_tp-1", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 hmass_tp-2", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 hmass_tp-3", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 umass_tp
        c.bench_function("SIMD Region 2 umass_tp-1", |b| {
            b.iter(|| iapws97::umass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 umass_tp-2", |b| {
            b.iter(|| iapws97::umass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 umass_tp-3", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 smass_tp
        c.bench_function("SIMD Region 2 smass_tp-1", |b| {
            b.iter(|| iapws97::smass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 smass_tp-2", |b| {
            b.iter(|| iapws97::smass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 smass_tp-3", |b| {
            b.iter(|| iapws97::smass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 cpmass_tp
        c.bench_function("SIMD Region 2 cpmass_tp-1", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 cpmass_tp-2", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 cpmass_tp-3", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 cvmass_tp
        c.bench_function("SIMD Region 2 cvmass_tp-1", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 cvmass_tp-2", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 cvmass_tp-3", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 vmass_tp
        c.bench_function("SIMD Region 2 vmass_tp-1", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 vmass_tp-2", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 vmass_tp-3", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 speed_sound_tp
        c.bench_function("SIMD Region 2 speed_sound_tp-1", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 speed_sound_tp-2", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("SIMD Region 2 speed_sound_tp-3", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 temperature_ps
        c.bench_function("SIMD Region 2 temperature_ps-1", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(0.001e6), black_box(3000.0)))
        });

        c.bench_function("SIMD Region 2 temperature_ps-2", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(3e6), black_box(3000.0)))
        });

        c.bench_function("SIMD Region 2 temperature_ps-3", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(3e6), black_box(4000.0)))
        });
        c.bench_function("SIMD Region 2 temperature_ps-4", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(5e6), black_box(3500.0)))
        });

        c.bench_function("SIMD Region 2 temperature_ps-5", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(5e6), black_box(4000.0)))
        });

        c.bench_function("SIMD Region 2 temperature_ps-6", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(25e6), black_box(3500.0)))
        });
        c.bench_function("SIMD Region 2 temperature_ps-7", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(40e6), black_box(2700.0)))
        });

        c.bench_function("SIMD Region 2 temperature_ps-8", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(60e6), black_box(2700.0)))
        });

        c.bench_function("SIMD Region 2 temperature_ps-9", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(60e6), black_box(3200.0)))
        });

        // Region 2 temperature_ph
        c.bench_function("SIMD Region 2 temperature_ph-1", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(0.1e6), black_box(7.5)))
        });

        c.bench_function("SIMD Region 2 temperature_ph-2", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(0.1e6), black_box(8.0)))
        });

        c.bench_function("SIMD Region 2 temperature_ph-3", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(2.5e6), black_box(8.0)))
        });
        c.bench_function("SIMD Region 2 temperature_ph-4", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(8e6), black_box(6.0)))
        });

        c.bench_function("SIMD Region 2 temperature_ph-5", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(8e6), black_box(7.5)))
        });

        c.bench_function("SIMD Region 2 temperature_ph-6", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(90e6), black_box(6.0)))
        });
        c.bench_function("SIMD Region 2 temperature_ph-7", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(20e6), black_box(5.75)))
        });

        c.bench_function("SIMD Region 2 temperature_ph-8", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(80e6), black_box(5.25)))
        });

        c.bench_function("SIMD Region 2 temperature_ph-9", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(80e6), black_box(5.75)))
        });
    } else {
        // Region 2 hmass_tp
        c.bench_function("Region 2 hmass_tp-1", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 hmass_tp-2", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 hmass_tp-3", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 umass_tp
        c.bench_function("Region 2 umass_tp-1", |b| {
            b.iter(|| iapws97::umass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 umass_tp-2", |b| {
            b.iter(|| iapws97::umass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 umass_tp-3", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 smass_tp
        c.bench_function("Region 2 smass_tp-1", |b| {
            b.iter(|| iapws97::smass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 smass_tp-2", |b| {
            b.iter(|| iapws97::smass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 smass_tp-3", |b| {
            b.iter(|| iapws97::smass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 cpmass_tp
        c.bench_function("Region 2 cpmass_tp-1", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 cpmass_tp-2", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 cpmass_tp-3", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 cvmass_tp
        c.bench_function("Region 2 cvmass_tp-1", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 cvmass_tp-2", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 cvmass_tp-3", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 vmass_tp
        c.bench_function("Region 2 vmass_tp-1", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 vmass_tp-2", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 vmass_tp-3", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 speed_sound_tp
        c.bench_function("Region 2 speed_sound_tp-1", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(300.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 speed_sound_tp-2", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(700.0), black_box(0.0035e6)))
        });

        c.bench_function("Region 2 speed_sound_tp-3", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(700.0), black_box(30e6)))
        });

        // Region 2 temperature_ps
        c.bench_function("Region 2 temperature_ps-1", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(0.001e6), black_box(3000.0)))
        });

        c.bench_function("Region 2 temperature_ps-2", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(3e6), black_box(3000.0)))
        });

        c.bench_function("Region 2 temperature_ps-3", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(3e6), black_box(4000.0)))
        });
        c.bench_function("Region 2 temperature_ps-4", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(5e6), black_box(3500.0)))
        });

        c.bench_function("Region 2 temperature_ps-5", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(5e6), black_box(4000.0)))
        });

        c.bench_function("Region 2 temperature_ps-6", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(25e6), black_box(3500.0)))
        });
        c.bench_function("Region 2 temperature_ps-7", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(40e6), black_box(2700.0)))
        });

        c.bench_function("Region 2 temperature_ps-8", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(60e6), black_box(2700.0)))
        });

        c.bench_function("Region 2 temperature_ps-9", |b| {
            b.iter(|| iapws97::temperature_ps(black_box(60e6), black_box(3200.0)))
        });

        // Region 2 temperature_ph
        c.bench_function("Region 2 temperature_ph-1", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(0.1e6), black_box(7.5)))
        });

        c.bench_function("Region 2 temperature_ph-2", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(0.1e6), black_box(8.0)))
        });

        c.bench_function("Region 2 temperature_ph-3", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(2.5e6), black_box(8.0)))
        });
        c.bench_function("Region 2 temperature_ph-4", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(8e6), black_box(6.0)))
        });

        c.bench_function("Region 2 temperature_ph-5", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(8e6), black_box(7.5)))
        });

        c.bench_function("Region 2 temperature_ph-6", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(90e6), black_box(6.0)))
        });
        c.bench_function("Region 2 temperature_ph-7", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(20e6), black_box(5.75)))
        });

        c.bench_function("Region 2 temperature_ph-8", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(80e6), black_box(5.25)))
        });

        c.bench_function("Region 2 temperature_ph-9", |b| {
            b.iter(|| iapws97::temperature_ph(black_box(80e6), black_box(5.75)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
