//! Moirai study-dispatch failures.

use core::fmt;

use moirai_core::error::ExecutorError;

/// Failure before or during Moirai trial dispatch.
#[derive(Debug)]
pub enum DispatchError {
    /// The compile-time chunk width is zero.
    ZeroChunkWidth,
    /// Caller-owned output storage does not match the study.
    OutputLength {
        /// Study sample count.
        expected: usize,
        /// Supplied slot count.
        actual: usize,
    },
    /// The borrowed Moirai executor rejected or failed scoped work.
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
