//! Equal-probability finite categories.
//!
//! The reduction follows Lemire, *Fast Random Integer Generation in an
//! Interval*, Section 4, Algorithm 5:
//! <https://arxiv.org/abs/1805.10941>.

use core::{marker::PhantomData, num::NonZeroUsize};

use crate::sampling::counter::{CategoricalSelection, Counter, Seed, StreamAlgorithm};

/// A validated non-zero number of categories.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct CategoryCount(NonZeroUsize);

impl CategoryCount {
    /// Construct a category count.
    #[must_use]
    pub const fn new(value: usize) -> Option<Self> {
        match NonZeroUsize::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Number of categories.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0.get()
    }
}

/// An index produced by a validated categorical distribution.
#[must_use]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct CategoryIndex(usize);

impl CategoryIndex {
    /// Validate a zero-based category offset against a category count.
    #[must_use]
    pub const fn new(value: usize, categories: CategoryCount) -> Option<Self> {
        if value < categories.get() {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Zero-based category offset.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }

    pub(super) const fn from_validated(value: usize) -> Self {
        Self(value)
    }
}

/// Equal-probability categorical sampler over counter algorithm `A`.
///
/// The sampler stores only its non-zero cardinality. Algorithm selection is a
/// zero-sized type parameter, and repeated random-access draws allocate
/// nothing.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Categorical<A> {
    categories: CategoryCount,
    algorithm: PhantomData<A>,
}

impl<A: StreamAlgorithm> Categorical<A> {
    /// Construct an equal-probability categorical sampler.
    ///
    /// # Examples
    ///
    /// ```
    /// use tyche_core::{Categorical, CategoryCount, Seed, SplitMix64};
    ///
    /// let sampler = Categorical::<SplitMix64>::new(
    ///     CategoryCount::new(6).expect("positive category count"),
    /// );
    /// assert!(sampler.at(Seed::new(7), 11).get() < 6);
    /// ```
    pub const fn new(categories: CategoryCount) -> Self {
        Self {
            categories,
            algorithm: PhantomData,
        }
    }

    /// Number of equiprobable categories.
    pub const fn category_count(self) -> CategoryCount {
        self.categories
    }

    /// Draw the category at a stable logical address.
    ///
    /// Multiply-high rejection gives every category the same number of
    /// accepted `u64` words. Rejected words advance only the dedicated retry
    /// coordinate for this address.
    ///
    /// # Panics
    ///
    /// Panics only if compiled for a target whose `usize` exceeds 64 bits, or
    /// if the multiply-high reduction violates its proven range invariant.
    pub fn at(self, seed: Seed, address: u64) -> CategoryIndex {
        let bound = u64::try_from(self.categories.get())
            .expect("invariant: Tyche supports targets with at most 64-bit usize");
        let rejection_threshold = bound.wrapping_neg() % bound;
        let mut attempt = 0_u64;

        loop {
            let word = Counter::<CategoricalSelection, A>::word(seed, address, attempt);
            let product = u128::from(word) * u128::from(bound);
            let low = u64::try_from(product & u128::from(u64::MAX))
                .expect("invariant: product low half is bounded by u64");
            if low >= rejection_threshold {
                let high = u64::try_from(product >> 64)
                    .expect("invariant: product high half is bounded by u64");
                let category = usize::try_from(high)
                    .expect("invariant: category is below the usize category count");
                return CategoryIndex::new(category, self.categories)
                    .expect("invariant: multiply-high result is below the category count");
            }
            attempt = attempt.wrapping_add(1);
        }
    }
}
