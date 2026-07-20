//! One indexed parameter-space sample.

/// An index-addressed study sample.
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

    /// Logical index within the study.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Borrow the fixed-width parameter values without copying.
    #[must_use]
    pub const fn values(&self) -> &[T; PARAMETERS] {
        &self.values
    }

    /// Consume the record and return its fixed-width values.
    #[must_use]
    pub fn into_values(self) -> [T; PARAMETERS] {
        self.values
    }
}
