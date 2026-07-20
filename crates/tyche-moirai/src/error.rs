//! Moirai study-dispatch failures.

use core::fmt;
use moirai_core::error::ExecutorError;
use tyche_core::SampleIndexError;

/// Failure before or during dispatch.
#[must_use]
#[non_exhaustive]
#[derive(Debug)]
pub enum DispatchError {
    /// Compile-time chunk width is zero.
    ZeroChunkWidth,
    /// Output storage does not match the study.
    OutputLength {
        /// Required slots.
        expected: usize,
        /// Supplied slots.
        actual: usize,
    },
    /// A design rejected an index below its declared sample count.
    DesignContract {
        /// First rejected logical sample index.
        index: usize,
        /// Sample count declared by the design.
        sample_count: usize,
    },
    /// Moirai scheduler failure.
    Executor(ExecutorError),
}

impl fmt::Display for DispatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroChunkWidth => formatter.write_str("Moirai chunk width must be positive"),
            Self::OutputLength { expected, actual } => write!(
                formatter,
                "study requires {expected} result slots but received {actual}"
            ),
            Self::DesignContract {
                index,
                sample_count,
            } => write!(
                formatter,
                "design rejected sample index {index} below its declared length {sample_count}"
            ),
            Self::Executor(error) => write!(formatter, "Moirai dispatch failed: {error}"),
        }
    }
}

impl std::error::Error for DispatchError {
    // Dynamic dispatch is required by std::error::Error's cold diagnostic seam.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Executor(error) => Some(error),
            Self::ZeroChunkWidth | Self::OutputLength { .. } | Self::DesignContract { .. } => None,
        }
    }
}

impl From<SampleIndexError> for DispatchError {
    fn from(error: SampleIndexError) -> Self {
        Self::DesignContract {
            index: error.index(),
            sample_count: error.sample_count(),
        }
    }
}

impl From<ExecutorError> for DispatchError {
    fn from(error: ExecutorError) -> Self {
        Self::Executor(error)
    }
}
