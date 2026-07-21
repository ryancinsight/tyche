//! Compile-time-dimensional Sobol design.

use super::{SobolDimensionError, SobolDimensions, SobolRange, SobolScramble, kernel};
use crate::sampling::{Design, SampleIndexError};

/// An allocation-free random-access Sobol design with fixed dimensions.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sobol<const PARAMETERS: usize, S> {
    range: SobolRange,
    scramble: S,
}

impl<const PARAMETERS: usize, S: SobolScramble> Sobol<PARAMETERS, S> {
    /// Construct a fixed-dimensional Sobol design.
    ///
    /// # Errors
    ///
    /// Returns [`SobolDimensionError`] unless `PARAMETERS` is in `1..=3`.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::num::NonZeroU32;
    /// use tyche_core::{Design, Sobol, SobolRange, Unscrambled};
    ///
    /// let range = SobolRange::new(0, NonZeroU32::new(8).unwrap()).unwrap();
    /// let design = Sobol::<2, Unscrambled>::new(range, Unscrambled).unwrap();
    /// let mut point = [0.0; 2];
    /// design.sample_unit_into(1, &mut point).unwrap();
    /// assert_eq!(point, [0.5, 0.5]);
    /// ```
    pub fn new(range: SobolRange, scramble: S) -> Result<Self, SobolDimensionError> {
        match SobolDimensions::new(PARAMETERS) {
            Ok(_) => Ok(Self { range, scramble }),
            Err(error) => Err(error),
        }
    }

    /// Configured contiguous sequence range.
    pub const fn range(&self) -> SobolRange {
        self.range
    }

    /// Scrambling policy.
    #[must_use]
    pub const fn scramble(&self) -> &S {
        &self.scramble
    }
}

impl<const PARAMETERS: usize, S: SobolScramble> Design<PARAMETERS> for Sobol<PARAMETERS, S> {
    fn sample_count(&self) -> usize {
        self.range.sample_count_usize()
    }

    fn sample_unit_into(
        &self,
        index: usize,
        output: &mut [f64; PARAMETERS],
    ) -> Result<(), SampleIndexError> {
        let point_index = self.range.point_index(index)?;
        kernel::sample(point_index, &self.scramble, output);
        Ok(())
    }
}
