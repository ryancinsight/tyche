//! One indexed parameter-space sample.

/// An index-addressed study sample.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Sample<T, const PARAMETERS: usize> {
    index: usize,
    values: [T; PARAMETERS],
}

impl<T, const PARAMETERS: usize> Sample<T, PARAMETERS> {
    pub(crate) const fn new(index: usize, values: [T; PARAMETERS]) -> Self {
        Self { index, values }
    }

    /// Logical index.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Borrow fixed-width values without copying.
    #[must_use]
    pub const fn values(&self) -> &[T; PARAMETERS] {
        &self.values
    }

    /// Consume the record into its values.
    #[must_use]
    pub fn into_values(self) -> [T; PARAMETERS] {
        self.values
    }
}
