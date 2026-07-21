//! Borrowed-or-owned validated discrete masses.

use alloc::borrow::Cow;

use super::{CategoryCount, CategoryIndex, WeightError};
use crate::sampling::SampleScalar;

/// A validated non-empty finite discrete mass table.
///
/// `Cow` gives borrowed callers a zero-copy path while accepting owned vectors
/// through the same constructor. Values are raw non-negative masses; callers
/// do not allocate or copy merely to normalize them.
#[must_use]
#[derive(Debug, Clone, PartialEq)]
pub struct DiscreteWeights<'a, T: Clone> {
    masses: Cow<'a, [T]>,
    total: T,
}

impl<'a, T: SampleScalar> DiscreteWeights<'a, T> {
    /// Validate borrowed or owned raw masses.
    ///
    /// # Errors
    ///
    /// Returns [`WeightError`] for an empty table, a non-finite or negative
    /// mass, a zero total, or native-precision total overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use tyche_core::DiscreteWeights;
    ///
    /// let borrowed = DiscreteWeights::new(Cow::Borrowed(&[1.0_f64, 3.0][..]))
    ///     .expect("valid masses");
    /// assert_eq!(borrowed.total(), 4.0);
    ///
    /// let owned = DiscreteWeights::new(Cow::Owned(vec![2.0_f64, 1.0]))
    ///     .expect("valid masses");
    /// assert_eq!(owned.category_count().get(), 2);
    /// ```
    pub fn new(masses: impl Into<Cow<'a, [T]>>) -> Result<Self, WeightError<T>> {
        let masses = masses.into();
        if masses.is_empty() {
            return Err(WeightError::Empty);
        }

        let mut total = T::ZERO;
        for (category, &mass) in masses.iter().enumerate() {
            if !mass.is_finite() {
                return Err(WeightError::NonFinite { category, mass });
            }
            if mass < T::ZERO {
                return Err(WeightError::Negative { category, mass });
            }
            total += mass;
            if !total.is_finite() {
                return Err(WeightError::NonFiniteTotal { total });
            }
        }
        if total == T::ZERO {
            return Err(WeightError::ZeroTotal);
        }

        Ok(Self { masses, total })
    }

    /// Number of categories.
    ///
    /// # Panics
    ///
    /// Panics only if the validated mass table's non-empty invariant is
    /// internally violated.
    pub fn category_count(&self) -> CategoryCount {
        CategoryCount::new(self.masses.len())
            .expect("invariant: validated discrete weights are non-empty")
    }

    /// Borrow the raw non-normalized masses.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.masses
    }

    /// Sum of raw masses in native scalar precision.
    #[must_use]
    pub const fn total(&self) -> T {
        self.total
    }

    /// Raw mass at a category produced by a categorical sampler.
    ///
    /// Returns `None` when the category came from a distribution with a larger
    /// support.
    #[must_use]
    pub fn mass(&self, category: CategoryIndex) -> Option<T> {
        self.masses.get(category.get()).copied()
    }

    /// Normalized probability at a category produced by a categorical sampler.
    ///
    /// Returns `None` when the category lies outside this table.
    #[must_use]
    pub fn probability(&self, category: CategoryIndex) -> Option<T> {
        self.mass(category).map(|mass| mass / self.total)
    }
}
