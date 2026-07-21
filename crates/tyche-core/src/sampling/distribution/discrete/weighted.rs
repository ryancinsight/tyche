//! Inverse-CDF sampling from finite non-negative masses.
//!
//! The generalized-inverse rule follows Devroye, *Non-Uniform Random Variate
//! Generation*, Chapter II, Section 2.1, Theorem 2.1:
//! <https://luc.devroye.org/chapter_two.pdf>.

use alloc::borrow::Cow;
use core::marker::PhantomData;

use super::{CategoryCount, CategoryIndex, DiscreteWeights, WeightError};
use crate::sampling::SampleScalar;
use crate::sampling::counter::{Counter, Seed, StreamAlgorithm, WeightedSelection};

/// Weighted finite categorical sampler over scalar `T` and algorithm `A`.
///
/// The sampler retains the caller's borrowed masses when constructed from a
/// borrowed slice. Construction validates in `O(K)` time, each draw applies a
/// linear inverse-CDF scan in `O(K)` time, and repeated draws allocate nothing.
/// Probabilities resolve on `T`'s finite native unit grid; no wider scalar is
/// used to conceal that quantization.
#[must_use]
#[derive(Debug, Clone, PartialEq)]
pub struct WeightedCategorical<'a, T: Clone, A> {
    weights: DiscreteWeights<'a, T>,
    algorithm: PhantomData<A>,
}

impl<'a, T: SampleScalar, A: StreamAlgorithm> WeightedCategorical<'a, T, A> {
    /// Validate masses and construct a weighted sampler.
    ///
    /// # Errors
    ///
    /// Returns [`WeightError`] when the masses do not form a valid finite
    /// distribution.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use tyche_core::{Seed, SplitMix64, WeightedCategorical};
    ///
    /// let sampler = WeightedCategorical::<f64, SplitMix64>::new(
    ///     Cow::Borrowed(&[0.0, 2.0, 0.0][..]),
    /// )
    /// .expect("valid masses");
    /// assert_eq!(sampler.at(Seed::new(3), 9).get(), 1);
    /// ```
    pub fn new(masses: impl Into<Cow<'a, [T]>>) -> Result<Self, WeightError<T>> {
        DiscreteWeights::new(masses).map(Self::from_validated)
    }

    pub(super) const fn from_validated(weights: DiscreteWeights<'a, T>) -> Self {
        Self {
            weights,
            algorithm: PhantomData,
        }
    }

    /// Validated mass table.
    pub const fn weights(&self) -> &DiscreteWeights<'a, T> {
        &self.weights
    }

    /// Number of weighted categories.
    pub fn category_count(&self) -> CategoryCount {
        self.weights.category_count()
    }

    /// Draw the weighted category at a stable logical address.
    pub fn at(&self, seed: Seed, address: u64) -> CategoryIndex {
        let threshold =
            Counter::<WeightedSelection, A>::unit::<T>(seed, address, 0) * self.weights.total();
        let mut cumulative = T::ZERO;
        let mut last_positive = 0;

        for (category, &mass) in self.weights.as_slice().iter().enumerate() {
            if mass > T::ZERO {
                last_positive = category;
                cumulative += mass;
                if threshold < cumulative {
                    return CategoryIndex::from_validated(category);
                }
            }
        }

        // Finite multiplication can round a value just below one to the total.
        // The generalized inverse then belongs to the final positive interval.
        CategoryIndex::from_validated(last_positive)
    }
}
