//! Validated relative Consus artifact keys.

use std::borrow::Cow;
use std::fmt;

/// A traversal-safe relative study-artifact key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ArtifactKey<'a>(Cow<'a, str>);

impl<'a> ArtifactKey<'a> {
    /// Validate a borrowed key.
    ///
    /// # Errors
    ///
    /// Rejects absolute paths, backslashes, empty segments, and `.` or `..`.
    pub fn borrowed(key: &'a str) -> Result<Self, ArtifactKeyError> {
        Self::new(Cow::Borrowed(key))
    }

    /// Validate an owned key.
    ///
    /// # Errors
    ///
    /// Rejects absolute paths, backslashes, empty segments, and `.` or `..`.
    pub fn owned(key: String) -> Result<Self, ArtifactKeyError> {
        Self::new(Cow::Owned(key))
    }

    fn new(key: Cow<'a, str>) -> Result<Self, ArtifactKeyError> {
        if key.is_empty() {
            return Err(ArtifactKeyError::Empty);
        }
        if key.starts_with('/') || key.starts_with('\\') || key.contains('\\') {
            return Err(ArtifactKeyError::AbsoluteOrPlatformPath);
        }
        for (index, segment) in key.split('/').enumerate() {
            if segment.is_empty() || segment == "." || segment == ".." {
                return Err(ArtifactKeyError::InvalidSegment { index });
            }
            if segment.contains(':') {
                return Err(ArtifactKeyError::PlatformPrefix { index });
            }
        }
        Ok(Self(key))
    }

    /// Relative key string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether storage is borrowed.
    #[must_use]
    pub const fn is_borrowed(&self) -> bool {
        matches!(self.0, Cow::Borrowed(_))
    }
}

/// Invalid artifact-key syntax.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactKeyError {
    /// Key is empty.
    Empty,
    /// Key is absolute or uses a platform separator.
    AbsoluteOrPlatformPath,
    /// A path segment is empty, `.` or `..`.
    InvalidSegment {
        /// Segment index.
        index: usize,
    },
    /// A segment resembles a Windows drive or URI prefix.
    PlatformPrefix {
        /// Segment index.
        index: usize,
    },
}

impl fmt::Display for ArtifactKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("artifact key must not be empty"),
            Self::AbsoluteOrPlatformPath => {
                formatter.write_str("artifact key must be a slash-separated relative path")
            }
            Self::InvalidSegment { index } => {
                write!(
                    formatter,
                    "artifact key segment {index} is empty or traversing"
                )
            }
            Self::PlatformPrefix { index } => {
                write!(
                    formatter,
                    "artifact key segment {index} has a platform prefix"
                )
            }
        }
    }
}

impl std::error::Error for ArtifactKeyError {}
