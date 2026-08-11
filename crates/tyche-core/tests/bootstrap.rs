//! Bootstrap resampling contracts.

use tyche_core::{Bootstrap, BootstrapError, Seed, SplitMix64};

const SEED: Seed = Seed::new(0x5459_4348_455F_4253);

#[test]
fn bootstrap_sizes_are_validated() {
    assert_eq!(
        Bootstrap::<SplitMix64>::new(0, 4),
        Err(BootstrapError::EmptyPopulation)
    );
    assert_eq!(
        Bootstrap::<SplitMix64>::new(4, 0),
        Err(BootstrapError::EmptyResample)
    );

    let design = Bootstrap::<SplitMix64>::new(7, 5).expect("positive sizes");
    assert_eq!(design.population_size(), 7);
    assert_eq!(design.resample_size(), 5);
}

#[test]
fn bootstrap_is_random_access_and_stays_in_population_support() {
    let design = Bootstrap::<SplitMix64>::new(7, 5).expect("positive sizes");
    let forward: Vec<_> = (0_u64..12).map(|draw| design.at(SEED, 3, draw)).collect();

    assert!(forward.iter().all(|&index| index < 7));
    for draw in (0_u64..12).rev() {
        assert_eq!(
            design.at(SEED, 3, draw),
            forward[usize::try_from(draw).expect("bounded test address")]
        );
    }
    assert_eq!(SplitMix64::VERSION.get(), 1, "known-answer stream version");
    let known_answer = Bootstrap::<SplitMix64>::new(5, 5).expect("known-answer sizes");
    assert_eq!(
        known_answer.at(Seed::new(42), 0, 0),
        1,
        "version-1 SplitMix64 bootstrap known-answer vector"
    );
    assert_eq!(
        (0_u64..5)
            .map(|draw| known_answer.at(Seed::new(42), 0, draw))
            .collect::<Vec<_>>(),
        vec![1, 0, 4, 3, 0],
        "version-1 SplitMix64 bootstrap replay vector"
    );
    assert_eq!(
        design.at(SEED, 2, 0),
        design.at(SEED, 2, 0),
        "repeated addresses remain stable"
    );
    let other_replicate: Vec<_> = (0_u64..12).map(|draw| design.at(SEED, 4, draw)).collect();
    assert_ne!(forward, other_replicate, "replicate separates the stream");
}

#[test]
fn bootstrap_fill_is_caller_owned_and_failure_atomic() {
    let design = Bootstrap::<SplitMix64>::new(11, 4).expect("positive sizes");
    let mut output = [usize::MAX; 4];
    design
        .fill_into(SEED, 9, &mut output)
        .expect("exact output length");
    assert!(output.iter().all(|&index| index < 11));

    let mut wrong_length = [usize::MAX; 3];
    assert_eq!(
        design.fill_into(SEED, 9, &mut wrong_length),
        Err(BootstrapError::OutputLength {
            expected: 4,
            actual: 3
        })
    );
    assert_eq!(wrong_length, [usize::MAX; 3]);
}

#[test]
fn bootstrap_fill_allocates_nothing_after_construction() {
    let design = Bootstrap::<SplitMix64>::new(31, 64).expect("positive sizes");
    let mut output = [0_usize; 64];
    let allocations = allocation_counter::measure(|| {
        design
            .fill_into(SEED, 12, &mut output)
            .expect("exact output length");
    });

    assert_eq!(allocations, allocation_counter::AllocationInfo::default());
    assert!(output.iter().all(|&index| index < 31));
}
