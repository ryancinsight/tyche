//! Discrete mass and support failures.

use core::fmt;

/// An invalid finite probability-mass table.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum WeightError<T> {
    /// The table contains no category.
    Empty,
    /// A mass is NaN or infinite.
    NonFinite {
        /// Offending category.
        category: usize,
        /// Offending mass.
        mass: T,
    },
    /// A mass is negative.
    Negative {
        /// Offending category.
        category: usize,
        /// Offending mass.
        mass: T,
    },
    /// Every category has zero mass.
    ZeroTotal,
    /// Finite inputs overflowed during native-precision accumulation.
    NonFiniteTotal {
        /// Accumulated non-finite total.
        total: T,
    },
}

impl<T: fmt::Debug> fmt::Display for WeightError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("discrete weights contain no category"),
            Self::NonFinite { category, mass } => {
                write!(
                    formatter,
                    "discrete weight {category} is non-finite: {mass:?}"
                )
            }
            Self::Negative { category, mass } => {
                write!(
                    formatter,
                    "discrete weight {category} is negative: {mass:?}"
                )
            }
            Self::ZeroTotal => formatter.write_str("discrete weights have zero total mass"),
            Self::NonFiniteTotal { total } => {
                write!(formatter, "discrete weight total is non-finite: {total:?}")
            }
        }
    }
}

impl<T: fmt::Debug> core::error::Error for WeightError<T> {}

/// An invalid target/proposal pair for discrete importance sampling.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum ImportanceError<T> {
    /// The target masses are invalid.
    TargetWeights(WeightError<T>),
    /// The proposal masses are invalid.
    ProposalWeights(WeightError<T>),
    /// Target and proposal have different finite supports.
    CategoryCountMismatch {
        /// Target category count.
        target: usize,
        /// Proposal category count.
        proposal: usize,
    },
    /// A positive target mass has zero proposal mass.
    MissingProposalSupport {
        /// Unsupported category.
        category: usize,
        /// Positive target mass.
        target_mass: T,
    },
    /// A representable target/proposal pair produces a non-finite ratio.
    NonFiniteLikelihoodRatio {
        /// Category whose ratio overflowed.
        category: usize,
        /// Normalized target probability.
        target_probability: T,
        /// Normalized proposal probability.
        proposal_probability: T,
    },
}

impl<T: fmt::Debug> fmt::Display for ImportanceError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TargetWeights(error) => write!(formatter, "invalid target weights: {error}"),
            Self::ProposalWeights(error) => write!(formatter, "invalid proposal weights: {error}"),
            Self::CategoryCountMismatch { target, proposal } => write!(
                formatter,
                "importance category count mismatch: target {target}, proposal {proposal}"
            ),
            Self::MissingProposalSupport {
                category,
                target_mass,
            } => write!(
                formatter,
                "proposal mass is zero at target category {category} with mass {target_mass:?}"
            ),
            Self::NonFiniteLikelihoodRatio {
                category,
                target_probability,
                proposal_probability,
            } => write!(
                formatter,
                "importance ratio is non-finite at category {category}: target probability {target_probability:?}, proposal probability {proposal_probability:?}"
            ),
        }
    }
}

impl<T: fmt::Debug> core::error::Error for ImportanceError<T> {}
