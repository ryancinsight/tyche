//! Counter-stream and design throughput instrument.
//!
//! Local evidence dated 2026-07-21 uses an Intel Core Ultra 9 285K (24 cores,
//! 24 logical processors) and the GNU x86-64 baseline target with SSE2/SSE3.
//! Criterion's default 3 s warm-up, 5 s measurement, and 100-sample analysis
//! report medians with 95% confidence intervals.

use core::num::NonZeroU32;
use std::borrow::Cow;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use tyche_core::{
    Categorical, CategoryCount, Counter, Design, DigitalShift, DiscreteImportance, LatinHypercube,
    RuntimeSobol, Seed, Sobol, SobolDimensions, SobolRange, SplitMix64, StandardNormal, UserDomain,
    WeightedCategorical,
};

type BenchmarkDomain = UserDomain<{ u64::from_le_bytes(*b"benchstr") }>;

fn counter_sampling(criterion: &mut Criterion) {
    let seed = Seed::new(0x5459_4348_455F_4245);
    let design =
        LatinHypercube::<8, SplitMix64>::new(seed, NonZeroU32::new(4_096).expect("positive"));
    let mut point = [0.0; 8];
    let sobol_range =
        SobolRange::new(1, NonZeroU32::new(4_096).expect("positive")).expect("bounded range");
    let shift = DigitalShift::<SplitMix64>::new(seed);
    let sobol = Sobol::<3, DigitalShift<SplitMix64>>::new(sobol_range, shift)
        .expect("three dimensions are supported");
    let runtime_sobol = RuntimeSobol::new(
        SobolDimensions::new(3).expect("three dimensions are supported"),
        sobol_range,
        shift,
    );
    let mut sobol_point = [0.0; 3];
    let mut sobol_rows = vec![0.0; 4_096 * 3].into_boxed_slice();
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
    group.throughput(Throughput::Elements(3));
    group.bench_function("sobol_fixed_width_3", |bencher| {
        bencher.iter(|| {
            index = index.wrapping_add(1) % 4_096;
            sobol
                .sample_unit_into(
                    usize::try_from(std::hint::black_box(index)).expect("bounded index"),
                    std::hint::black_box(&mut sobol_point),
                )
                .expect("bounded index");
            sobol_point
        });
    });
    group.bench_function("sobol_runtime_width_3", |bencher| {
        bencher.iter(|| {
            index = index.wrapping_add(1) % 4_096;
            runtime_sobol
                .sample_unit_slice_into(
                    usize::try_from(std::hint::black_box(index)).expect("bounded index"),
                    std::hint::black_box(&mut sobol_point),
                )
                .expect("bounded index and output width");
            sobol_point
        });
    });
    group.throughput(Throughput::Elements(4_096 * 3));
    group.bench_function("sobol_runtime_rows_width_3", |bencher| {
        bencher.iter(|| {
            runtime_sobol
                .sample_unit_rows_into(std::hint::black_box(&mut sobol_rows))
                .expect("exact row-major output length");
            let index = usize::try_from(index).expect("bounded index") % sobol_rows.len();
            sobol_rows[std::hint::black_box(index)]
        });
    });
    group.finish();
}

fn discrete_sampling(criterion: &mut Criterion) {
    let seed = Seed::new(0x5459_4348_455F_4245);
    let categorical =
        Categorical::<SplitMix64>::new(CategoryCount::new(16).expect("positive category count"));
    let proposal_masses = [
        1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
    ];
    let target_masses = [
        2.0_f64, 1.0, 4.0, 3.0, 6.0, 5.0, 8.0, 7.0, 7.0, 8.0, 5.0, 6.0, 3.0, 4.0, 1.0, 2.0,
    ];
    let weighted = WeightedCategorical::<f64, SplitMix64>::new(Cow::Borrowed(&proposal_masses[..]))
        .expect("valid masses");
    let importance = DiscreteImportance::<f64, SplitMix64>::new(
        Cow::Borrowed(&target_masses[..]),
        Cow::Borrowed(&proposal_masses[..]),
    )
    .expect("compatible target and proposal");
    let mut index = 0_u64;

    let mut group = criterion.benchmark_group("discrete_sampling");
    group.throughput(Throughput::Elements(1));
    group.bench_function("categorical_width_16", |bencher| {
        bencher.iter(|| {
            index = index.wrapping_add(1);
            categorical.at(std::hint::black_box(seed), std::hint::black_box(index))
        });
    });
    group.bench_function("weighted_width_16", |bencher| {
        bencher.iter(|| {
            index = index.wrapping_add(1);
            weighted.at(std::hint::black_box(seed), std::hint::black_box(index))
        });
    });
    group.bench_function("importance_width_16", |bencher| {
        bencher.iter(|| {
            index = index.wrapping_add(1);
            importance.at(std::hint::black_box(seed), std::hint::black_box(index))
        });
    });
    group.finish();
}

criterion_group!(benches, counter_sampling, discrete_sampling);
criterion_main!(benches);
