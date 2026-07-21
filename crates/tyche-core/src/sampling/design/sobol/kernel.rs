//! One const-generic random-access coordinate kernel.

use super::{SobolScramble, direction::DIRECTIONS, policy};

const UNIT_SCALE: f64 = 1.0 / 4_294_967_296.0;

pub(super) fn sample<const PARAMETERS: usize, S: SobolScramble>(
    point_index: u32,
    scramble: &S,
    output: &mut [f64; PARAMETERS],
) {
    let gray_code = point_index ^ (point_index >> 1);
    let mut numerators = [0; PARAMETERS];
    for (dimension, numerator) in numerators.iter_mut().enumerate() {
        *numerator = policy::shift(scramble, dimension);
    }

    let mut active_bits = gray_code;
    while active_bits != 0 {
        let bit = usize::try_from(active_bits.trailing_zeros())
            .expect("invariant: a u32 bit position fits in usize");
        for (dimension, numerator) in numerators.iter_mut().enumerate() {
            *numerator ^= DIRECTIONS[dimension][bit];
        }
        active_bits &= active_bits - 1;
    }

    for (coordinate, numerator) in output.iter_mut().zip(numerators) {
        *coordinate = f64::from(numerator) * UNIT_SCALE;
    }
}
