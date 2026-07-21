//! Reproducibility seed value.

/// Reproducibility seed for an uncertainty study.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Seed(u64);

impl Seed {
    /// Construct a seed from its stable bit representation.
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    /// Stable bit representation.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }
}
