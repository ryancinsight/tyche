//! Checked index conversions shared by random-access designs.

/// Convert a design coordinate into the counter's index domain.
pub(in crate::sampling) fn counter_coordinate(value: usize) -> u64 {
    let Ok(value) = u64::try_from(value) else {
        unreachable!("invariant: Tyche supports targets with at most 64-bit usize");
    };
    value
}

/// Convert a validated 32-bit design index into the host index domain.
pub(in crate::sampling) fn design_index(value: u32) -> usize {
    let Ok(value) = usize::try_from(value) else {
        unreachable!("invariant: Tyche requires a target with at least 32-bit usize");
    };
    value
}

/// Convert a modulo-reduced counter result into a stratum index.
pub(in crate::sampling) fn stratum_index(value: u64) -> u32 {
    let Ok(value) = u32::try_from(value) else {
        unreachable!("invariant: value is reduced modulo a u32 sample count");
    };
    value
}
