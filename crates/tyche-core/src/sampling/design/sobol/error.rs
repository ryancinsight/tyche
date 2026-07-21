//! Typed Sobol construction and sampling failures.

use core::{fmt, num::NonZeroU32};

use crate::sampling::SampleIndexError;

/// An unsupported runtime or compile-time Sobol dimension count.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SobolDimensionError {
    requested: usize,
    maximum: usize,
}

impl SobolDimensionError {
    pub(super) const fn new(requested: usize, maximum: usize) -> Self {
        Self { requested, maximum }
    }

    /// Requested dimension count.
    #[must_use]
    pub const fn requested(self) -> usize {
        self.requested
    }

    /// Largest supported dimension count.
    #[must_use]
    pub const fn maximum(self) -> usize {
        self.maximum
    }
}

impl fmt::Display for SobolDimensionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Sobol dimensions must be in 1..={}, received {}",
            self.maximum, self.requested
        )
    }
}

impl core::error::Error for SobolDimensionError {}

/// A Sobol sequence range exceeds the 32-bit direction-number address space.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SobolRangeError {
    start: u32,
    sample_count: NonZeroU32,
}

impl SobolRangeError {
    pub(super) const fn new(start: u32, sample_count: NonZeroU32) -> Self {
        Self {
            start,
            sample_count,
        }
    }

    /// First requested sequence index.
    #[must_use]
    pub const fn start(self) -> u32 {
        self.start
    }

    /// Requested number of points.
    #[must_use]
    pub const fn sample_count(self) -> NonZeroU32 {
        self.sample_count
    }
}

impl fmt::Display for SobolRangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Sobol range starting at {} with {} points exceeds the 32-bit index space",
            self.start, self.sample_count
        )
    }
}

impl core::error::Error for SobolRangeError {}

/// A runtime-dimensional design could not fill the requested output.
#[must_use]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeSampleError {
    /// The requested sample is outside the configured sequence range.
    SampleIndex(SampleIndexError),
    /// The caller-provided output slice has the wrong dimension count.
    OutputLength {
        /// Required number of coordinates.
        expected: usize,
        /// Supplied number of coordinates.
        actual: usize,
    },
    /// A row-major output buffer has the wrong total length.
    OutputMatrixLength {
        /// Number of rows required by the sequence range.
        sample_count: usize,
        /// Number of coordinates required per row.
        dimensions: usize,
        /// Supplied total number of scalar slots.
        actual: usize,
    },
}

impl fmt::Display for RuntimeSampleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SampleIndex(error) => error.fmt(formatter),
            Self::OutputLength { expected, actual } => write!(
                formatter,
                "runtime design requires {expected} output coordinates, received {actual}"
            ),
            Self::OutputMatrixLength {
                sample_count,
                dimensions,
                actual,
            } => write!(
                formatter,
                "runtime design requires {sample_count} rows of {dimensions} coordinates, received {actual} scalar slots"
            ),
        }
    }
}

impl core::error::Error for RuntimeSampleError {}

impl From<SampleIndexError> for RuntimeSampleError {
    fn from(error: SampleIndexError) -> Self {
        Self::SampleIndex(error)
    }
}
