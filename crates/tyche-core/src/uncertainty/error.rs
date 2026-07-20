//! Conformal calibration failures.

use core::fmt;

/// Invalid level or score set.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConformalError<T> {
    /// Miscoverage is outside `(0,1)`.
    InvalidMiscoverage(T),
    /// No scores.
    EmptyScores,
    /// A score is negative or non-finite.
    InvalidScore {
        /// Score index.
        index: usize,
        /// Invalid value.
        value: T,
    },
}

impl<T: fmt::Debug> fmt::Display for ConformalError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMiscoverage(value) => write!(
                formatter,
                "miscoverage must satisfy 0 < alpha < 1, got {value:?}"
            ),
            Self::EmptyScores => formatter.write_str("conformal calibration needs scores"),
            Self::InvalidScore { index, value } => {
                write!(formatter, "score {index} is invalid: {value:?}")
            }
        }
    }
}

impl<T: fmt::Debug + 'static> core::error::Error for ConformalError<T> {}
