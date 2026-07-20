//! Statistical sample-count failures.

use core::fmt;

/// A statistic received fewer observations than its mathematical definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InsufficientSamples {
    required: u64,
    actual: u64,
}

impl InsufficientSamples {
    /// Construct a sample-count failure.
    #[must_use]
    pub const fn new(required: u64, actual: u64) -> Self {
        Self { required, actual }
    }

    /// Minimum number of observations.
    #[must_use]
    pub const fn required(self) -> u64 {
        self.required
    }

    /// Supplied number of observations.
    #[must_use]
    pub const fn actual(self) -> u64 {
        self.actual
    }
}

impl fmt::Display for InsufficientSamples {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "statistic requires {} samples but received {}",
            self.required, self.actual
        )
    }
}

impl core::error::Error for InsufficientSamples {}
