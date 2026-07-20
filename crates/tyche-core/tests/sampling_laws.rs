//! Sampling-law evidence.

use core::num::NonZeroUsize;
use tyche_core::{Design, LatinHypercube, Seed, SplitMix64, StandardNormal};

#[test]
fn every_dimension_contains_every_stratum_once() {
    const PARAMETERS: usize = 8;
    const SAMPLES: usize = 97;
    let design = LatinHypercube::<PARAMETERS>::new(
        Seed::new(0x5459_4348_455F_4C48),
        NonZeroUsize::new(SAMPLES).expect("positive"),
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
        LatinHypercube::<3>::new(Seed::new(42), NonZeroUsize::new(SAMPLES).expect("positive"));
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
    let seed = Seed::new(7);
    for index in 0..10_000_u64 {
        assert!((0.0..1.0).contains(&SplitMix64::unit(seed, index, 0)));
        let open = SplitMix64::open_unit(seed, index, 1);
        assert!(open > 0.0 && open < 1.0);
        assert!(StandardNormal::at(seed, index, 2).is_finite());
    }
}

proptest::proptest! {
    #[test]
    fn points_stay_in_unit_hypercube(seed in proptest::prelude::any::<u64>(), count in 1_usize..256) {
        let design = LatinHypercube::<4>::new(Seed::new(seed), NonZeroUsize::new(count).expect("positive"));
        for index in 0..count {
            let mut point = [0.0; 4];
            design.sample_unit_into(index, &mut point).expect("valid");
            proptest::prop_assert!(point.into_iter().all(|value| (0.0..1.0).contains(&value)));
        }
    }
}
