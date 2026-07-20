//! Conformal calibration failures.

use core::fmt;

/// Invalid calibration level or score set.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConformalError<T> {
    /// Miscoverage must lie strictly between zero and one.
    InvalidMiscoverage(T),
    /// Calibration requires at least one score.
    EmptyScores,
    /// Every nonconformity score must be finite and non-negative.
    InvalidScore {
        /// Index of the invalid score.
        index: usize,
        /// Invalid score.
        value: T,
    },
}

impl<T: fmt::Debug> fmt::Display for ConformalError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMiscoverage(value) => write!(
                formatter,
                "conformal miscoverage must satisfy 0 < alpha < 1, got {value:?}"
            ),
            Self::EmptyScores => formatter.write_str("conformal calibration needs scores"),
            Self::InvalidScore { index, value } => {
                write!(
                    formatter,
                    "score {index} is not finite and non-negative: {value:?}"
                )
            }
        }
    }
}

impl<T: fmt::Debug + 'static> core::error::Error for ConformalError<T> {}
