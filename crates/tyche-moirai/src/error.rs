//! Moirai study-dispatch failures.

use core::fmt;
use moirai_core::error::ExecutorError;

/// Failure before or during dispatch.
#[must_use]
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
            Self::Executor(error) => write!(formatter, "Moirai dispatch failed: {error}"),
        }
    }
}

impl std::error::Error for DispatchError {}

impl From<ExecutorError> for DispatchError {
    fn from(error: ExecutorError) -> Self {
        Self::Executor(error)
    }
}
