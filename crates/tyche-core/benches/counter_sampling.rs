//! Counter-stream and design throughput instrument.

use core::num::NonZeroU32;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use tyche_core::{Counter, Design, LatinHypercube, Seed, SplitMix64, StandardNormal, UserDomain};

type BenchmarkDomain = UserDomain<{ u64::from_le_bytes(*b"benchstr") }>;

fn counter_sampling(criterion: &mut Criterion) {
    let seed = Seed::new(0x5459_4348_455F_4245);
    let design =
        LatinHypercube::<8, SplitMix64>::new(seed, NonZeroU32::new(4_096).expect("positive"));
    let mut point = [0.0; 8];
    let mut index = 0_u64;

    let mut group = criterion.benchmark_group("counter_sampling");
    group.throughput(Throughput::Elements(1));
    group.bench_function("word", |bencher| {
        bencher.iter(|| {
            index = index.wrapping_add(1);
            Counter::<BenchmarkDomain, SplitMix64>::word(
                std::hint::black_box(seed),
                std::hint::black_box(index),
                0,
            )
        });
    });
    group.bench_function("normal", |bencher| {
        bencher.iter(|| {
            index = index.wrapping_add(1);
            StandardNormal::<f64, SplitMix64>::at(
                std::hint::black_box(seed),
                std::hint::black_box(index),
                0,
            )
        });
    });
    group.throughput(Throughput::Elements(8));
    group.bench_function("latin_hypercube_width_8", |bencher| {
        bencher.iter(|| {
            index = index.wrapping_add(1) % 4_096;
            design
                .sample_unit_into(
                    usize::try_from(std::hint::black_box(index)).expect("bounded index"),
                    std::hint::black_box(&mut point),
                )
                .expect("bounded index");
            point
        });
    });
    group.finish();
}

criterion_group!(benches, counter_sampling);
criterion_main!(benches);
