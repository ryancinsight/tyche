//! Runtime-dimensional Sobol design with caller-owned output storage.

use super::{RuntimeSampleError, SobolDimensions, SobolRange, SobolScramble, kernel};

/// An allocation-free Sobol view for a runtime-selected dimension count.
///
/// Runtime selection dispatches once per operation to one of the three
/// const-generic kernels. Coordinate loops remain monomorphic and the caller
/// owns the output slice.
///
/// # Examples
///
/// ```
/// use core::num::NonZeroU32;
/// use tyche_core::{RuntimeSobol, SobolDimensions, SobolRange, Unscrambled};
///
/// let design = RuntimeSobol::new(
///     SobolDimensions::new(2).unwrap(),
///     SobolRange::new(0, NonZeroU32::new(2).unwrap()).unwrap(),
///     Unscrambled,
/// );
/// let mut rows = [f64::NAN; 4];
/// design.sample_unit_rows_into(&mut rows).unwrap();
/// assert_eq!(rows, [0.0, 0.0, 0.5, 0.5]);
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuntimeSobol<S> {
    dimensions: SobolDimensions,
    range: SobolRange,
    scramble: S,
}

impl<S: SobolScramble> RuntimeSobol<S> {
    /// Construct a runtime-dimensional design from validated dimensions.
    pub const fn new(dimensions: SobolDimensions, range: SobolRange, scramble: S) -> Self {
        Self {
            dimensions,
            range,
            scramble,
        }
    }

    /// Validated dimension count.
    pub const fn dimensions(&self) -> SobolDimensions {
        self.dimensions
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

    /// Number of sample points.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.range.sample_count_usize()
    }

    /// Fill a caller-owned slice with one normalized point.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeSampleError::OutputLength`] when `output.len()` does
    /// not equal [`Self::dimensions`], or [`RuntimeSampleError::SampleIndex`]
    /// when `index >= self.sample_count()`.
    pub fn sample_unit_slice_into(
        &self,
        index: usize,
        output: &mut [f64],
    ) -> Result<(), RuntimeSampleError> {
        match self.dimensions.get() {
            1 => sample_dimension::<1, S>(self.range, &self.scramble, index, output)?,
            2 => sample_dimension::<2, S>(self.range, &self.scramble, index, output)?,
            3 => sample_dimension::<3, S>(self.range, &self.scramble, index, output)?,
            _ => unreachable!("invariant: SobolDimensions permits only one through three"),
        }
        Ok(())
    }

    /// Fill a caller-owned row-major buffer with the complete design.
    ///
    /// Dimension dispatch and output validation occur once for the complete
    /// range. Rows are contiguous and no temporary point or collection is
    /// allocated.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeSampleError::OutputMatrixLength`] unless `output`
    /// contains exactly `self.sample_count() * self.dimensions().get()` scalar
    /// slots. The output remains unchanged when validation fails.
    pub fn sample_unit_rows_into(&self, output: &mut [f64]) -> Result<(), RuntimeSampleError> {
        let sample_count = self.sample_count();
        let dimensions = self.dimensions.get();
        let Some(expected) = sample_count.checked_mul(dimensions) else {
            return Err(RuntimeSampleError::OutputMatrixLength {
                sample_count,
                dimensions,
                actual: output.len(),
            });
        };
        if output.len() != expected {
            return Err(RuntimeSampleError::OutputMatrixLength {
                sample_count,
                dimensions,
                actual: output.len(),
            });
        }

        match dimensions {
            1 => sample_rows::<1, S>(self.range, &self.scramble, output)?,
            2 => sample_rows::<2, S>(self.range, &self.scramble, output)?,
            3 => sample_rows::<3, S>(self.range, &self.scramble, output)?,
            _ => unreachable!("invariant: SobolDimensions permits only one through three"),
        }
        Ok(())
    }
}

fn sample_dimension<const PARAMETERS: usize, S: SobolScramble>(
    range: SobolRange,
    scramble: &S,
    index: usize,
    output: &mut [f64],
) -> Result<(), RuntimeSampleError> {
    let actual = output.len();
    let Ok(output) = <&mut [f64; PARAMETERS]>::try_from(output) else {
        return Err(RuntimeSampleError::OutputLength {
            expected: PARAMETERS,
            actual,
        });
    };
    let point_index = range.point_index(index)?;
    kernel::sample(point_index, scramble, output);
    Ok(())
}

fn sample_rows<const PARAMETERS: usize, S: SobolScramble>(
    range: SobolRange,
    scramble: &S,
    output: &mut [f64],
) -> Result<(), RuntimeSampleError> {
    let sample_count = range.sample_count_usize();
    let actual = output.len();
    let mut point_index = range.start();
    for row in output.chunks_exact_mut(PARAMETERS) {
        let Ok(row) = <&mut [f64; PARAMETERS]>::try_from(row) else {
            return Err(RuntimeSampleError::OutputMatrixLength {
                sample_count,
                dimensions: PARAMETERS,
                actual,
            });
        };
        kernel::sample(point_index, scramble, row);
        point_index = point_index.wrapping_add(1);
    }
    Ok(())
}
