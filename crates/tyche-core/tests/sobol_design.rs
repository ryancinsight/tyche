//! Sobol analytical, differential, boundary, and replay contracts.

use core::num::NonZeroU32;

use tyche_core::{
    Design, DigitalShift, RuntimeSampleError, RuntimeSobol, Seed, Sobol, SobolDimensions,
    SobolRange, SobolScramble, SplitMix64, Unscrambled,
};

const EXPECTED_FIRST_EIGHT: [[f64; 3]; 8] = [
    [0.0, 0.0, 0.0],
    [0.5, 0.5, 0.5],
    [0.75, 0.25, 0.25],
    [0.25, 0.75, 0.75],
    [0.375, 0.375, 0.625],
    [0.875, 0.875, 0.125],
    [0.625, 0.125, 0.875],
    [0.125, 0.625, 0.375],
];

fn count(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value).expect("test count is positive")
}

fn range(start: u32, sample_count: u32) -> SobolRange {
    SobolRange::new(start, count(sample_count)).expect("test range is representable")
}

#[test]
fn canonical_first_three_dimensions_match_known_vectors() {
    let design = Sobol::<3, Unscrambled>::new(range(0, 8), Unscrambled)
        .expect("three dimensions are supported");

    for (index, expected) in EXPECTED_FIRST_EIGHT.iter().enumerate() {
        let mut actual = [f64::NAN; 3];
        design
            .sample_unit_into(index, &mut actual)
            .expect("vector index is in range");
        assert_eq!(actual.map(f64::to_bits), expected.map(f64::to_bits));
    }
}

#[test]
fn random_access_matches_independent_sequential_recurrence() {
    let design = Sobol::<3, Unscrambled>::new(range(0, 1_024), Unscrambled)
        .expect("three dimensions are supported");
    let directions = oracle_directions();
    let mut state = [0_u32; 3];

    for index in 0..1_024_usize {
        if index != 0 {
            let sequence_index = u32::try_from(index).expect("bounded test index");
            let bit = usize::try_from(sequence_index.trailing_zeros()).expect("bit fits usize");
            for dimension in 0..3 {
                state[dimension] ^= directions[dimension][bit];
            }
        }
        let expected = state.map(|numerator| f64::from(numerator) / 4_294_967_296.0);
        let mut actual = [f64::NAN; 3];
        design
            .sample_unit_into(index, &mut actual)
            .expect("test index is in range");
        assert_eq!(
            actual.map(f64::to_bits),
            expected.map(f64::to_bits),
            "sequence index {index}"
        );
    }
}

#[test]
fn origin_aligned_power_of_two_prefix_stratifies_every_projection() {
    const SAMPLE_COUNT: u32 = 256;
    let sequence_range = range(0, SAMPLE_COUNT);
    let plain = Sobol::<3, Unscrambled>::new(sequence_range, Unscrambled)
        .expect("three dimensions are supported");
    let shifted = Sobol::<3, DigitalShift<SplitMix64>>::new(
        sequence_range,
        DigitalShift::new(Seed::new(0x4E45_545F_5052_4F4A)),
    )
    .expect("three dimensions are supported");

    assert_projection_stratification(&plain, SAMPLE_COUNT);
    assert_projection_stratification(&shifted, SAMPLE_COUNT);
}

fn assert_projection_stratification<S: SobolScramble>(design: &Sobol<3, S>, sample_count: u32) {
    for dimension in 0..3 {
        let mut coordinates =
            Vec::with_capacity(usize::try_from(sample_count).expect("sample count fits usize"));
        for index in 0..design.sample_count() {
            let mut point = [0.0; 3];
            design
                .sample_unit_into(index, &mut point)
                .expect("test index is in range");
            coordinates.push(point[dimension]);
        }
        coordinates.sort_by(f64::total_cmp);

        for (stratum, coordinate) in coordinates.into_iter().enumerate() {
            let stratum = u32::try_from(stratum).expect("bounded stratum");
            let lower = f64::from(stratum) / f64::from(sample_count);
            let upper = f64::from(stratum + 1) / f64::from(sample_count);
            assert!(
                (lower..upper).contains(&coordinate),
                "dimension {dimension}, stratum {stratum}, coordinate {coordinate}"
            );
        }
    }
}

#[test]
fn runtime_dispatch_matches_fixed_kernels_for_every_supported_dimension() {
    let sequence_range = range(1, 64);
    let shift = DigitalShift::<SplitMix64>::new(Seed::new(0x534F_424F_4C5F_5254));

    compare_runtime::<1>(sequence_range, shift);
    compare_runtime::<2>(sequence_range, shift);
    compare_runtime::<3>(sequence_range, shift);
}

#[test]
fn replay_is_order_independent_and_seed_sensitive() {
    let sequence_range = range(1, 32);
    let first =
        Sobol::<3, DigitalShift<SplitMix64>>::new(sequence_range, DigitalShift::new(Seed::new(17)))
            .expect("three dimensions are supported");
    let second =
        Sobol::<3, DigitalShift<SplitMix64>>::new(sequence_range, DigitalShift::new(Seed::new(18)))
            .expect("three dimensions are supported");
    let mut forward = [[0.0; 3]; 32];
    let mut reverse = [[0.0; 3]; 32];

    for (index, point) in forward.iter_mut().enumerate() {
        first
            .sample_unit_into(index, point)
            .expect("test index is in range");
    }
    for (index, point) in reverse.iter_mut().enumerate().rev() {
        first
            .sample_unit_into(index, point)
            .expect("test index is in range");
    }
    assert_eq!(
        forward.map(|point| point.map(f64::to_bits)),
        reverse.map(|point| point.map(f64::to_bits))
    );

    let mut other_seed = [0.0; 3];
    second
        .sample_unit_into(0, &mut other_seed)
        .expect("test index is in range");
    assert_ne!(forward[0].map(f64::to_bits), other_seed.map(f64::to_bits));
}

#[test]
fn validated_domains_report_exact_boundary_failures() {
    let zero = SobolDimensions::new(0).expect_err("zero dimensions are unsupported");
    assert_eq!(zero.requested(), 0);
    assert_eq!(zero.maximum(), 3);
    let four = SobolDimensions::new(4).expect_err("four dimensions are unsupported");
    assert_eq!(four.requested(), 4);
    assert_eq!(four.maximum(), 3);

    let overflow = SobolRange::new(u32::MAX, count(2)).expect_err("range must overflow");
    assert_eq!(overflow.start(), u32::MAX);
    assert_eq!(overflow.sample_count(), count(2));
    assert!(SobolRange::new(u32::MAX, count(1)).is_ok());

    let fixed_zero = Sobol::<0, Unscrambled>::new(range(0, 1), Unscrambled)
        .expect_err("zero dimensions are unsupported");
    assert_eq!(fixed_zero.requested(), 0);
    assert_eq!(fixed_zero.maximum(), 3);
}

#[test]
fn runtime_failures_preserve_the_caller_buffer() {
    let design = RuntimeSobol::new(
        SobolDimensions::new(3).expect("three dimensions are supported"),
        range(1, 2),
        Unscrambled,
    );
    let mut short = [7.0, 11.0];
    let short_error = design
        .sample_unit_slice_into(0, &mut short)
        .expect_err("short output must fail");
    assert_eq!(
        short_error,
        RuntimeSampleError::OutputLength {
            expected: 3,
            actual: 2
        }
    );
    assert_eq!(short.map(f64::to_bits), [7.0_f64, 11.0].map(f64::to_bits));

    let mut point = [7.0, 11.0, 13.0];
    let index_error = design
        .sample_unit_slice_into(2, &mut point)
        .expect_err("index equal to count must fail");
    let RuntimeSampleError::SampleIndex(index_error) = index_error else {
        panic!("expected sample-index error");
    };
    assert_eq!(index_error.index(), 2);
    assert_eq!(index_error.sample_count(), 2);
    assert_eq!(
        point.map(f64::to_bits),
        [7.0_f64, 11.0, 13.0].map(f64::to_bits)
    );

    let mut incomplete_rows = [17.0; 5];
    let matrix_error = design
        .sample_unit_rows_into(&mut incomplete_rows)
        .expect_err("incomplete row-major output must fail");
    assert_eq!(
        matrix_error,
        RuntimeSampleError::OutputMatrixLength {
            sample_count: 2,
            dimensions: 3,
            actual: 5,
        }
    );
    assert_eq!(
        incomplete_rows.map(f64::to_bits),
        [17.0_f64; 5].map(f64::to_bits)
    );
}

#[test]
fn row_major_fill_matches_individual_runtime_samples() {
    let design = RuntimeSobol::new(
        SobolDimensions::new(3).expect("three dimensions are supported"),
        range(1, 64),
        DigitalShift::<SplitMix64>::new(Seed::new(73)),
    );
    let mut rows = [0.0; 64 * 3];
    design
        .sample_unit_rows_into(&mut rows)
        .expect("row-major output has the exact required length");

    for (index, row) in rows.chunks_exact(3).enumerate() {
        let mut point = [0.0; 3];
        design
            .sample_unit_slice_into(index, &mut point)
            .expect("point index and output are valid");
        assert!(
            row.iter()
                .zip(point)
                .all(|(left, right)| left.to_bits() == right.to_bits())
        );
    }
}

#[test]
fn balance_qualification_is_explicit() {
    assert!(range(0, 256).is_origin_aligned_power_of_two());
    assert!(!range(1, 256).is_origin_aligned_power_of_two());
    assert!(!range(0, 255).is_origin_aligned_power_of_two());
}

fn compare_runtime<const PARAMETERS: usize>(
    sequence_range: SobolRange,
    shift: DigitalShift<SplitMix64>,
) {
    let fixed = Sobol::<PARAMETERS, DigitalShift<SplitMix64>>::new(sequence_range, shift)
        .expect("test dimensions are supported");
    let runtime = RuntimeSobol::new(
        SobolDimensions::new(PARAMETERS).expect("test dimensions are supported"),
        sequence_range,
        shift,
    );
    let mut fixed_point = [0.0; PARAMETERS];
    let mut runtime_point = [0.0; PARAMETERS];

    for index in 0..fixed.sample_count() {
        fixed
            .sample_unit_into(index, &mut fixed_point)
            .expect("test index is in range");
        runtime
            .sample_unit_slice_into(index, &mut runtime_point)
            .expect("runtime output is correctly sized");
        assert_eq!(
            runtime_point.map(f64::to_bits),
            fixed_point.map(f64::to_bits)
        );
    }
}

fn oracle_directions() -> [[u32; 32]; 3] {
    let mut directions = [[0; 32]; 3];
    for (bit, direction) in directions[0].iter_mut().enumerate() {
        *direction = 1_u32 << (31 - bit);
    }
    directions[1] = oracle_parameterized_directions(1, 0, &[1]);
    directions[2] = oracle_parameterized_directions(2, 1, &[1, 3]);
    directions
}

fn oracle_parameterized_directions(degree: usize, coefficients: u32, initial: &[u32]) -> [u32; 32] {
    assert_eq!(initial.len(), degree);
    let mut directions = [0; 32];
    for bit in 0..degree {
        directions[bit] = initial[bit] << (31 - bit);
    }
    for bit in degree..32 {
        let mut direction = directions[bit - degree] ^ (directions[bit - degree] >> degree);
        for term in 1..degree {
            if (coefficients >> (degree - 1 - term)) & 1 == 1 {
                direction ^= directions[bit - term];
            }
        }
        directions[bit] = direction;
    }
    directions
}
