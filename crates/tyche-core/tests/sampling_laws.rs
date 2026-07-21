//! Sampling-law evidence.

use core::num::NonZeroU32;
use tyche_core::{
    Counter, Design, LatinHypercube, SampleScalar, Seed, SplitMix64, StandardNormal, UserDomain,
};

#[test]
fn every_dimension_contains_every_stratum_once() {
    const PARAMETERS: usize = 8;
    const SAMPLES: usize = 97;
    let design = LatinHypercube::<PARAMETERS, SplitMix64>::new(
        Seed::new(0x5459_4348_455F_4C48),
        NonZeroU32::new(97).expect("positive"),
    );
    for dimension in 0..PARAMETERS {
        let mut seen = [false; SAMPLES];
        for sample in 0..SAMPLES {
            let stratum = design.stratum(sample, dimension).expect("valid");
            assert!(!seen[stratum]);
            seen[stratum] = true;
        }
        assert!(seen.into_iter().all(core::convert::identity));
    }
}

#[test]
fn replay_is_bitwise_and_order_independent() {
    const SAMPLES: usize = 31;
    let design =
        LatinHypercube::<3, SplitMix64>::new(Seed::new(42), NonZeroU32::new(31).expect("positive"));
    let mut forward = [[0.0_f64; 3]; SAMPLES];
    for (index, point) in forward.iter_mut().enumerate() {
        design.sample_unit_into(index, point).expect("valid");
    }
    for index in (0..SAMPLES).rev() {
        let mut replay = [0.0; 3];
        design.sample_unit_into(index, &mut replay).expect("valid");
        assert_eq!(replay.map(f64::to_bits), forward[index].map(f64::to_bits));
    }
}

#[test]
fn streams_respect_domains() {
    type TestStream = UserDomain<{ u64::from_le_bytes(*b"testvect") }>;

    let seed = Seed::new(7);
    for index in 0..10_000_u64 {
        let unit = Counter::<TestStream, SplitMix64>::unit::<f64>(seed, index, 0);
        assert!((0.0..1.0).contains(&unit));
        let open = Counter::<TestStream, SplitMix64>::open_unit::<f64>(seed, index, 1);
        assert!(open > 0.0 && open < 1.0);
    }
}

#[test]
fn standard_normal_contract_holds_for_every_supported_primitive_field() {
    fn assert_contract<T: SampleScalar>() {
        let seed = Seed::new(7);
        for index in 0..10_000_u64 {
            let value = StandardNormal::<T, SplitMix64>::at(seed, index, 2);
            let replay = StandardNormal::<T, SplitMix64>::at(seed, index, 2);
            assert!(<T as eunomia::NumericElement>::is_finite(value));
            assert!(value == replay);
        }
    }

    assert_contract::<f32>();
    assert_contract::<f64>();
}

#[test]
fn standard_normal_empirical_moments_match_the_target_distribution() {
    const COUNT: u64 = 100_000;
    let seed = Seed::new(0x5459_4348_455F_4E4F);
    let mut sum = 0.0;
    let mut square_sum = 0.0;
    for index in 0..COUNT {
        let value = StandardNormal::<f64, SplitMix64>::at(seed, index, 0);
        sum += value;
        square_sum = value.mul_add(value, square_sum);
    }

    let count = f64::from(u32::try_from(COUNT).expect("bounded sample count"));
    let mean = sum / count;
    let variance = square_sum / count - mean * mean;
    let mean_bound = 6.0 / count.sqrt();
    let variance_bound = 6.0 * (2.0 / (count - 1.0)).sqrt();

    // Empirical evidence only: under independent normal draws these are
    // six-standard-error bounds for the mean and variance estimators.
    assert!(mean.abs() <= mean_bound, "mean {mean} exceeds {mean_bound}");
    assert!(
        (variance - 1.0).abs() <= variance_bound,
        "variance {variance} exceeds unit-variance bound {variance_bound}"
    );
}

proptest::proptest! {
    #[test]
    fn points_stay_in_unit_hypercube(seed in proptest::prelude::any::<u64>(), count in 1_u16..256) {
        let design = LatinHypercube::<4, SplitMix64>::new(
            Seed::new(seed),
            NonZeroU32::new(u32::from(count)).expect("positive"),
        );
        for index in 0..usize::from(count) {
            let mut point = [0.0; 4];
            design.sample_unit_into(index, &mut point).expect("valid");
            proptest::prop_assert!(point.into_iter().all(|value| (0.0..1.0).contains(&value)));
        }
    }
}
