use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_steam::iapws97;

pub fn criterion_benchmark(c: &mut Criterion) {
    if cfg!(feature = "nightly") {
        // Region 3 hmass_tp
        c.bench_function("SIMD Region 3 hmass_tp-1", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("SIMD Region 3 hmass_tp-2", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("SIMD Region 3 hmass_tp-3", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 umass_tp
        c.bench_function("SIMD Region 3 umass_tp-1", |b| {
            b.iter(|| iapws97::umass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("SIMD Region 3 umass_tp-2", |b| {
            b.iter(|| iapws97::umass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("SIMD Region 3 umass_tp-3", |b| {
            b.iter(|| iapws97::umass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 smass_tp
        c.bench_function("SIMD Region 3 smass_tp-1", |b| {
            b.iter(|| iapws97::smass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("SIMD Region 3 smass_tp-2", |b| {
            b.iter(|| iapws97::smass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("SIMD Region 3 smass_tp-3", |b| {
            b.iter(|| iapws97::smass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 cpmass_tp
        c.bench_function("SIMD Region 3 cpmass_tp-1", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("SIMD Region 3 cpmass_tp-2", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("SIMD Region 3 cpmass_tp-3", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 cvmass_tp
        c.bench_function("SIMD Region 3 cvmass_tp-1", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("SIMD Region 3 cvmass_tp-2", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("SIMD Region 3 cvmass_tp-3", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 vmass_tp
        c.bench_function("SIMD Region 3 vmass_tp-1", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("SIMD Region 3 vmass_tp-2", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("SIMD Region 3 vmass_tp-3", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 speed_sound_tp
        c.bench_function("SIMD Region 3 speed_sound_tp-1", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("SIMD Region 3 speed_sound_tp-2", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("SIMD Region 3 speed_sound_tp-3", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(750.0), black_box(78.3095639e6)))
        });
    } else {
        // Region 3 hmass_tp
        c.bench_function("Region 3 hmass_tp-1", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("Region 3 hmass_tp-2", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("Region 3 hmass_tp-3", |b| {
            b.iter(|| iapws97::hmass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 umass_tp
        c.bench_function("Region 3 umass_tp-1", |b| {
            b.iter(|| iapws97::umass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("Region 3 umass_tp-2", |b| {
            b.iter(|| iapws97::umass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("Region 3 umass_tp-3", |b| {
            b.iter(|| iapws97::umass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 smass_tp
        c.bench_function("Region 3 smass_tp-1", |b| {
            b.iter(|| iapws97::smass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("Region 3 smass_tp-2", |b| {
            b.iter(|| iapws97::smass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("Region 3 smass_tp-3", |b| {
            b.iter(|| iapws97::smass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 cpmass_tp
        c.bench_function("Region 3 cpmass_tp-1", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("Region 3 cpmass_tp-2", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("Region 3 cpmass_tp-3", |b| {
            b.iter(|| iapws97::cpmass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 cvmass_tp
        c.bench_function("Region 3 cvmass_tp-1", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("Region 3 cvmass_tp-2", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("Region 3 cvmass_tp-3", |b| {
            b.iter(|| iapws97::cvmass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 vmass_tp
        c.bench_function("Region 3 vmass_tp-1", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("Region 3 vmass_tp-2", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("Region 3 vmass_tp-3", |b| {
            b.iter(|| iapws97::vmass_tp(black_box(750.0), black_box(78.3095639e6)))
        });

        // Region 3 speed_sound_tp
        c.bench_function("Region 3 speed_sound_tp-1", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(650.0), black_box(25.5837018e6)))
        });

        c.bench_function("Region 3 speed_sound_tp-2", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(650.0), black_box(22.2930643e6)))
        });

        c.bench_function("Region 3 speed_sound_tp-3", |b| {
            b.iter(|| iapws97::speed_sound_tp(black_box(750.0), black_box(78.3095639e6)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
