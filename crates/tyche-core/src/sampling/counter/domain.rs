//! Compile-time stream-domain separation.

use core::fmt;

mod private {
    pub trait Sealed {}
}

/// A compile-time identity separating one random stream from another.
///
/// Tyche seals this trait so every downstream domain is represented by
/// [`UserDomain<TAG>`]. Equal tags are the same Rust type; unequal tags
/// produce distinct words at equal seed, index, and draw coordinates.
pub trait StreamDomain: private::Sealed {
    /// Stable domain tag included in the counter key.
    const TAG: u64;
}

/// A downstream-defined stream domain identified by a stable numeric tag.
///
/// The tag is part of the replay contract and must not change after results
/// have been persisted. A domain should own one tag in its defining crate.
#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserDomain<const TAG: u64>;

impl<const TAG: u64> private::Sealed for UserDomain<TAG> {}

impl<const TAG: u64> StreamDomain for UserDomain<TAG> {
    const TAG: u64 = TAG;
}

impl<const TAG: u64> fmt::Debug for UserDomain<TAG> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("UserDomain")
            .field(&format_args!("{TAG:#018x}"))
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::sampling) struct LatinHypercubeStride;

impl private::Sealed for LatinHypercubeStride {}

impl StreamDomain for LatinHypercubeStride {
    const TAG: u64 = u64::from_le_bytes(*b"lhsstrid");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::sampling) struct LatinHypercubeOffset;

impl private::Sealed for LatinHypercubeOffset {}

impl StreamDomain for LatinHypercubeOffset {
    const TAG: u64 = u64::from_le_bytes(*b"lhsoffst");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::sampling) struct LatinHypercubeJitter;

impl private::Sealed for LatinHypercubeJitter {}

impl StreamDomain for LatinHypercubeJitter {
    const TAG: u64 = u64::from_le_bytes(*b"lhsjittr");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::sampling) struct StandardNormalRadius;

impl private::Sealed for StandardNormalRadius {}

impl StreamDomain for StandardNormalRadius {
    const TAG: u64 = u64::from_le_bytes(*b"normradi");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::sampling) struct StandardNormalAngle;

impl private::Sealed for StandardNormalAngle {}

impl StreamDomain for StandardNormalAngle {
    const TAG: u64 = u64::from_le_bytes(*b"normangl");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::sampling) struct SobolDigitalShift;

impl private::Sealed for SobolDigitalShift {}

impl StreamDomain for SobolDigitalShift {
    const TAG: u64 = u64::from_le_bytes(*b"sobolshf");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::sampling) struct CategoricalSelection;

impl private::Sealed for CategoricalSelection {}

impl StreamDomain for CategoricalSelection {
    const TAG: u64 = u64::from_le_bytes(*b"category");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::sampling) struct WeightedSelection;

impl private::Sealed for WeightedSelection {}

impl StreamDomain for WeightedSelection {
    const TAG: u64 = u64::from_le_bytes(*b"weighted");
}

const _: () = {
    let tags = [
        LatinHypercubeStride::TAG,
        LatinHypercubeOffset::TAG,
        LatinHypercubeJitter::TAG,
        StandardNormalRadius::TAG,
        StandardNormalAngle::TAG,
        SobolDigitalShift::TAG,
        CategoricalSelection::TAG,
        WeightedSelection::TAG,
    ];
    let mut left = 0;
    while left < tags.len() {
        let mut right = left + 1;
        while right < tags.len() {
            assert!(tags[left] != tags[right]);
            right += 1;
        }
        left += 1;
    }
};
