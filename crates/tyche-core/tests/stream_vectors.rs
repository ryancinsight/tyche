//! Bitwise replay vectors for the public stream contract.

use core::num::NonZeroU32;

use tyche_core::{Counter, Design, LatinHypercube, Seed, SplitMix64, StandardNormal, UserDomain};

type VectorDomain = UserDomain<{ u64::from_le_bytes(*b"testvect") }>;
type OtherDomain = UserDomain<{ u64::from_le_bytes(*b"othervct") }>;

#[test]
fn counter_words_and_units_match_stream_version() {
    let seed = Seed::new(0x5459_4348_455F_5354);
    let words = [
        Counter::<VectorDomain, SplitMix64>::word(seed, 0, 0),
        Counter::<VectorDomain, SplitMix64>::word(seed, 1, 0),
        Counter::<VectorDomain, SplitMix64>::word(seed, 0, 1),
        Counter::<VectorDomain, SplitMix64>::word(seed, u64::MAX, u64::MAX),
    ];
    let single_units = [
        Counter::<VectorDomain, SplitMix64>::unit::<f32>(seed, 0, 0).to_bits(),
        Counter::<VectorDomain, SplitMix64>::unit::<f32>(seed, 1, 0).to_bits(),
    ];
    let double_units = [
        Counter::<VectorDomain, SplitMix64>::unit::<f64>(seed, 0, 1).to_bits(),
        Counter::<VectorDomain, SplitMix64>::open_unit::<f64>(seed, u64::MAX, u64::MAX).to_bits(),
    ];

    assert_eq!(Counter::<VectorDomain, SplitMix64>::VERSION.get(), 1);
    assert_eq!(tyche_core::StreamVersion::new(0), None);
    assert_eq!(
        tyche_core::StreamVersion::new(1),
        Some(Counter::<VectorDomain, SplitMix64>::VERSION)
    );
    assert_eq!(
        words,
        [
            16_474_956_665_820_689_217,
            7_713_741_379_062_511_766,
            11_279_972_423_469_089_642,
            3_749_879_984_441_801_135,
        ]
    );
    assert_eq!(single_units, [1_063_559_885, 1_054_218_604]);
    assert_eq!(
        double_units,
        [4_603_683_018_580_173_432, 4_596_492_004_635_148_316]
    );
}

#[test]
fn equal_coordinates_are_separated_by_domain() {
    let seed = Seed::new(0x5459_4348_455F_444F);
    for index in [0, 1, 2, u64::MAX] {
        for draw in [0, 1, 2, u64::MAX] {
            assert_ne!(
                Counter::<VectorDomain, SplitMix64>::word(seed, index, draw),
                Counter::<OtherDomain, SplitMix64>::word(seed, index, draw),
            );
        }
    }
}

#[test]
fn normal_values_match_stream_version_in_each_precision() {
    let seed = Seed::new(0x5459_4348_455F_4E4F);
    let single = [
        StandardNormal::<f32, SplitMix64>::at(seed, 0, 0).to_bits(),
        StandardNormal::<f32, SplitMix64>::at(seed, 1, 0).to_bits(),
        StandardNormal::<f32, SplitMix64>::at(seed, 0, 1).to_bits(),
    ];
    let double = [
        StandardNormal::<f64, SplitMix64>::at(seed, 0, 0).to_bits(),
        StandardNormal::<f64, SplitMix64>::at(seed, 1, 0).to_bits(),
        StandardNormal::<f64, SplitMix64>::at(seed, 0, 1).to_bits(),
    ];

    assert_eq!(single, [3_203_414_633, 1_068_663_080, 3_213_358_985]);
    assert_eq!(
        double,
        [
            13_825_495_935_330_735_777,
            4_608_959_388_051_692_144,
            13_830_834_766_438_358_553,
        ]
    );
}

#[test]
fn latin_hypercube_points_match_stream_version() {
    let design = LatinHypercube::<3, SplitMix64>::new(
        Seed::new(0x5459_4348_455F_4C48),
        NonZeroU32::new(8).expect("positive"),
    );
    let mut bits = [[0_u64; 3]; 4];
    for (index, point_bits) in bits.iter_mut().enumerate() {
        let mut point = [0.0; 3];
        design.sample_unit_into(index, &mut point).expect("valid");
        *point_bits = point.map(f64::to_bits);
    }

    assert_eq!(
        bits,
        [
            [
                4_603_387_366_448_315_294,
                4_603_632_457_832_350_012,
                4_597_153_305_453_772_824,
            ],
            [
                4_603_972_240_827_896_389,
                4_603_935_407_968_540_598,
                4_603_773_653_272_417_852,
            ],
            [
                4_605_518_185_013_383_706,
                4_605_102_667_200_475_048,
                4_606_332_643_312_028_750,
            ],
            [
                4_606_896_691_720_083_223,
                4_606_534_087_373_538_175,
                4_599_525_736_358_451_261,
            ],
        ]
    );
}
