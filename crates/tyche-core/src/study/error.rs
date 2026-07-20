//! Study construction failures.

use core::fmt;

/// Reproducible-study construction failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudyError {
    /// A study needs a stable non-empty name.
    EmptyName,
}

impl fmt::Display for StudyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("study name must not be empty"),
        }
    }
}

impl core::error::Error for StudyError {}
