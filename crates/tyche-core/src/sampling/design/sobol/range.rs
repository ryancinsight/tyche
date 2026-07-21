//! Validated dimensions and contiguous sequence ranges.

use core::num::NonZeroU32;

use super::{SobolDimensionError, SobolRangeError, direction::MAX_DIMENSIONS};
use crate::sampling::SampleIndexError;

/// Validated runtime Sobol dimension count.
///
/// # Examples
///
/// ```
/// use tyche_core::SobolDimensions;
///
/// let dimensions = SobolDimensions::new(3).unwrap();
/// assert_eq!(dimensions.get(), 3);
/// assert!(SobolDimensions::new(4).is_err());
/// ```
#[must_use]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SobolDimensions(usize);

impl SobolDimensions {
    /// Validate a runtime dimension count.
    ///
    /// # Errors
    ///
    /// Returns [`SobolDimensionError`] unless `dimensions` is in `1..=3`.
    pub const fn new(dimensions: usize) -> Result<Self, SobolDimensionError> {
        if dimensions == 0 || dimensions > MAX_DIMENSIONS {
            return Err(SobolDimensionError::new(dimensions, MAX_DIMENSIONS));
        }
        Ok(Self(dimensions))
    }

    /// Validated dimension count.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

/// A validated contiguous range of 32-bit Sobol sequence indices.
///
/// Starting at zero exposes the canonical origin. Starting at one exposes the
/// conventional first non-origin point without pretending that a scramble
/// seed is a skip count. The strongest `(t, m, s)` net balance guarantee
/// applies to an origin-aligned prefix whose count is a power of two.
///
/// # Examples
///
/// ```
/// use core::num::NonZeroU32;
/// use tyche_core::SobolRange;
///
/// let range = SobolRange::new(0, NonZeroU32::new(256).unwrap()).unwrap();
/// assert!(range.is_origin_aligned_power_of_two());
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SobolRange {
    start: u32,
    sample_count: NonZeroU32,
}

impl SobolRange {
    /// Validate a contiguous sequence range.
    ///
    /// # Errors
    ///
    /// Returns [`SobolRangeError`] if the last requested point would exceed
    /// [`u32::MAX`].
    pub const fn new(start: u32, sample_count: NonZeroU32) -> Result<Self, SobolRangeError> {
        if start.checked_add(sample_count.get() - 1).is_none() {
            return Err(SobolRangeError::new(start, sample_count));
        }
        Ok(Self {
            start,
            sample_count,
        })
    }

    /// First sequence index.
    #[must_use]
    pub const fn start(self) -> u32 {
        self.start
    }

    /// Number of points in the range.
    #[must_use]
    pub const fn sample_count(self) -> NonZeroU32 {
        self.sample_count
    }

    /// Whether this range is an origin-aligned power-of-two prefix.
    #[must_use]
    pub const fn is_origin_aligned_power_of_two(self) -> bool {
        self.start == 0 && self.sample_count.get().is_power_of_two()
    }

    pub(super) fn point_index(self, index: usize) -> Result<u32, SampleIndexError> {
        let sample_count = self.sample_count_usize();
        if index >= sample_count {
            return Err(SampleIndexError::new(index, sample_count));
        }
        let Ok(offset) = u32::try_from(index) else {
            unreachable!("invariant: validated sample offset is bounded by a u32 count");
        };
        Ok(self.start + offset)
    }

    pub(super) fn sample_count_usize(self) -> usize {
        usize_from_u32(self.sample_count.get())
    }
}

fn usize_from_u32(value: u32) -> usize {
    match usize::try_from(value) {
        Ok(value) => value,
        Err(_) => unreachable!("invariant: Tyche requires a target with at least 32-bit usize"),
    }
}
