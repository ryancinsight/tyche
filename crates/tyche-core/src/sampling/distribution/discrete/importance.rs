//! Likelihood-ratio samples over finite target and proposal masses.
//!
//! The support law and expectation identity follow Owen, *Monte Carlo Theory,
//! Methods and Examples*, Chapter 9, Section 9.1, Theorem 9.1:
//! <https://artowen.su.domains/mc/Ch-var-is.pdf>.

use alloc::borrow::Cow;

use super::{CategoryIndex, DiscreteWeights, ImportanceError, WeightedCategorical};
use crate::sampling::{SampleScalar, Seed, StreamAlgorithm};

/// One discrete proposal draw and its target/proposal likelihood ratio.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImportanceSample<T> {
    category: CategoryIndex,
    likelihood_ratio: T,
}

impl<T: SampleScalar> ImportanceSample<T> {
    /// Sampled proposal category.
    pub const fn category(self) -> CategoryIndex {
        self.category
    }

    /// Target probability divided by proposal probability at the category.
    #[must_use]
    pub const fn likelihood_ratio(self) -> T {
        self.likelihood_ratio
    }

    /// Apply the likelihood ratio to a domain response.
    #[must_use]
    pub fn weigh(self, response: T) -> T {
        response * self.likelihood_ratio
    }
}

/// Discrete importance sampler over scalar `T` and stream algorithm `A`.
///
/// Target and proposal masses use one `Cow`-based contract. Construction
/// validates the support law and every representable likelihood ratio once;
/// repeated random-access draws are infallible and allocation-free.
#[must_use]
#[derive(Debug, Clone, PartialEq)]
pub struct DiscreteImportance<'a, T: Clone, A> {
    target: DiscreteWeights<'a, T>,
    proposal: WeightedCategorical<'a, T, A>,
}

impl<'a, T: SampleScalar, A: StreamAlgorithm> DiscreteImportance<'a, T, A> {
    /// Validate target and proposal masses.
    ///
    /// # Errors
    ///
    /// Returns [`ImportanceError`] when either mass table is invalid, their
    /// category counts differ, proposal support omits positive target mass, or
    /// a likelihood ratio is not finite in `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use tyche_core::{DiscreteImportance, Seed, SplitMix64};
    ///
    /// let sampler = DiscreteImportance::<f64, SplitMix64>::new(
    ///     Cow::Borrowed(&[1.0, 3.0][..]),
    ///     Cow::Borrowed(&[2.0, 2.0][..]),
    /// )
    /// .expect("compatible target and proposal");
    /// let sample = sampler.at(Seed::new(5), 17);
    /// assert!(matches!(sample.category().get(), 0 | 1));
    /// assert!(matches!(sample.likelihood_ratio(), 0.5 | 1.5));
    /// ```
    pub fn new(
        target: impl Into<Cow<'a, [T]>>,
        proposal: impl Into<Cow<'a, [T]>>,
    ) -> Result<Self, ImportanceError<T>> {
        let target = DiscreteWeights::new(target).map_err(ImportanceError::TargetWeights)?;
        let proposal = DiscreteWeights::new(proposal).map_err(ImportanceError::ProposalWeights)?;

        let target_count = target.category_count().get();
        let proposal_count = proposal.category_count().get();
        if target_count != proposal_count {
            return Err(ImportanceError::CategoryCountMismatch {
                target: target_count,
                proposal: proposal_count,
            });
        }

        for (category, (&target_mass, &proposal_mass)) in target
            .as_slice()
            .iter()
            .zip(proposal.as_slice())
            .enumerate()
        {
            if target_mass > T::ZERO && proposal_mass == T::ZERO {
                return Err(ImportanceError::MissingProposalSupport {
                    category,
                    target_mass,
                });
            }
            if proposal_mass > T::ZERO {
                let target_probability = target_mass / target.total();
                let proposal_probability = proposal_mass / proposal.total();
                let ratio = target_probability / proposal_probability;
                if !ratio.is_finite() {
                    return Err(ImportanceError::NonFiniteLikelihoodRatio {
                        category,
                        target_probability,
                        proposal_probability,
                    });
                }
            }
        }

        Ok(Self {
            target,
            proposal: WeightedCategorical::from_validated(proposal),
        })
    }

    /// Validated target masses.
    pub const fn target(&self) -> &DiscreteWeights<'a, T> {
        &self.target
    }

    /// Validated proposal sampler.
    pub const fn proposal(&self) -> &WeightedCategorical<'a, T, A> {
        &self.proposal
    }

    /// Target/proposal likelihood ratio at a category.
    ///
    /// Returns `None` when the category is outside this distribution or has
    /// zero proposal probability. A zero-proposal category is never sampled;
    /// construction already rejects positive target mass at such a category.
    #[must_use]
    pub fn likelihood_ratio(&self, category: CategoryIndex) -> Option<T> {
        let target_mass = self.target.mass(category)?;
        let proposal_weights = self.proposal.weights();
        let proposal_mass = proposal_weights.mass(category)?;
        if proposal_mass == T::ZERO {
            return None;
        }

        let target_probability = target_mass / self.target.total();
        let proposal_probability = proposal_mass / proposal_weights.total();
        Some(target_probability / proposal_probability)
    }

    /// Draw a proposal category and its target/proposal likelihood ratio.
    ///
    /// # Panics
    ///
    /// Panics only if the validated proposal sampler returns a category with
    /// zero proposal mass, which would violate its inverse-CDF invariant.
    pub fn at(&self, seed: Seed, address: u64) -> ImportanceSample<T> {
        let category = self.proposal.at(seed, address);
        ImportanceSample {
            category,
            likelihood_ratio: self
                .likelihood_ratio(category)
                .expect("invariant: a proposal draw has positive proposal mass"),
        }
    }
}
