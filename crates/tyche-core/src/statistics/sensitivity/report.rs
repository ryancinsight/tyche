//! Unified sensitivity reports shared by the online estimators.

use core::marker::PhantomData;

use eunomia::RealField;

mod private {
    /// Seals [`SensitivityKind`] to Tyche's statistic families.
    pub trait Sealed {}
}

/// A family of sensitivity statistics carried by one [`Report`].
///
/// The three families select, by zero-sized marker, how many of the report's
/// statistic channels are populated: correlation uses one, Sobol' two, and
/// Morris three. The kind is otherwise a pure type-level tag; it never
/// affects layout or arithmetic.
pub trait SensitivityKind: private::Sealed + Copy {
    /// Number of populated statistic channels.
    const CHANNELS: usize;
}

/// Squared Pearson-correlation family.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CorrelationKind;

impl private::Sealed for CorrelationKind {}

impl SensitivityKind for CorrelationKind {
    const CHANNELS: usize = 1;
}

/// Saltelli first- and total-order Sobol' family.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SobolKind;

impl private::Sealed for SobolKind {}

impl SensitivityKind for SobolKind {
    const CHANNELS: usize = 2;
}

/// Morris `mu`, `mu_star`, and `sigma` family.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorrisKind;

impl private::Sealed for MorrisKind {}

impl SensitivityKind for MorrisKind {
    const CHANNELS: usize = 3;
}

/// Largest statistic-channel count across the kinds.
const MAX_CHANNELS: usize = 3;

/// Statistics produced by an online sensitivity estimator.
///
/// `Kind` names the statistic family and fixes the meaning of each populated
/// channel; `channels` stores them with a shared, kind-independent shape.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Report<Kind: SensitivityKind, T, const PARAMETERS: usize, const OUTPUTS: usize> {
    sample_count: u64,
    channels: [[[T; PARAMETERS]; OUTPUTS]; MAX_CHANNELS],
    kind: PhantomData<Kind>,
}

impl<Kind: SensitivityKind, T, const PARAMETERS: usize, const OUTPUTS: usize>
    Report<Kind, T, PARAMETERS, OUTPUTS>
{
    /// Construct a report from a populated channel table.
    pub(crate) const fn new(
        sample_count: u64,
        channels: [[[T; PARAMETERS]; OUTPUTS]; MAX_CHANNELS],
    ) -> Self {
        Self {
            sample_count,
            channels,
            kind: PhantomData,
        }
    }

    /// Observations folded into the report.
    #[must_use]
    pub const fn sample_count(&self) -> u64 {
        self.sample_count
    }

    /// Borrow one statistic channel indexed by output.
    #[must_use]
    pub const fn by_output(&self, channel: usize) -> &[[T; PARAMETERS]; OUTPUTS] {
        &self.channels[channel]
    }
}

impl<Kind: SensitivityKind, T, const PARAMETERS: usize> Report<Kind, T, PARAMETERS, 1> {
    /// Borrow one statistic channel for the single output.
    #[must_use]
    pub const fn single(&self, channel: usize) -> &[T; PARAMETERS] {
        &self.channels[channel][0]
    }
}

/// Squared Pearson-correlation screening report.
pub type SensitivityReport<T, const PARAMETERS: usize, const OUTPUTS: usize = 1> =
    Report<CorrelationKind, T, PARAMETERS, OUTPUTS>;

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize>
    SensitivityReport<T, PARAMETERS, OUTPUTS>
{
    /// Wrap populated squared Pearson indices in the shared report shape.
    pub(crate) fn from_correlations(
        sample_count: u64,
        squared_correlations: [[T; PARAMETERS]; OUTPUTS],
    ) -> Self {
        Self::new(
            sample_count,
            [
                squared_correlations,
                [[T::ZERO; PARAMETERS]; OUTPUTS],
                [[T::ZERO; PARAMETERS]; OUTPUTS],
            ],
        )
    }
}

impl<T, const PARAMETERS: usize, const OUTPUTS: usize> SensitivityReport<T, PARAMETERS, OUTPUTS> {
    /// Borrow the squared Pearson indices for every output.
    #[must_use]
    pub const fn squared_correlations_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        self.by_output(0)
    }
}

impl<T, const PARAMETERS: usize> SensitivityReport<T, PARAMETERS, 1> {
    /// Borrow the single-output squared Pearson indices.
    #[must_use]
    pub const fn squared_correlations(&self) -> &[T; PARAMETERS] {
        self.single(0)
    }
}

/// Saltelli Sobol' index report.
pub type SobolReport<T, const PARAMETERS: usize, const OUTPUTS: usize = 1> =
    Report<SobolKind, T, PARAMETERS, OUTPUTS>;

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize>
    SobolReport<T, PARAMETERS, OUTPUTS>
{
    /// Wrap populated first- and total-order indices in the shared report shape.
    pub(crate) fn from_indices(
        sample_count: u64,
        first_order: [[T; PARAMETERS]; OUTPUTS],
        total_order: [[T; PARAMETERS]; OUTPUTS],
    ) -> Self {
        Self::new(
            sample_count,
            [first_order, total_order, [[T::ZERO; PARAMETERS]; OUTPUTS]],
        )
    }
}

impl<T, const PARAMETERS: usize, const OUTPUTS: usize> SobolReport<T, PARAMETERS, OUTPUTS> {
    /// First-order indices `S_i` for every output.
    #[must_use]
    pub const fn first_order_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        self.by_output(0)
    }

    /// Total-order indices `S_Ti` for every output.
    #[must_use]
    pub const fn total_order_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        self.by_output(1)
    }
}

impl<T, const PARAMETERS: usize> SobolReport<T, PARAMETERS, 1> {
    /// First-order indices for the single output.
    #[must_use]
    pub const fn first_order(&self) -> &[T; PARAMETERS] {
        self.single(0)
    }

    /// Total-order indices for the single output.
    #[must_use]
    pub const fn total_order(&self) -> &[T; PARAMETERS] {
        self.single(1)
    }
}

/// Morris elementary-effect screening report.
pub type MorrisReport<T, const PARAMETERS: usize, const OUTPUTS: usize = 1> =
    Report<MorrisKind, T, PARAMETERS, OUTPUTS>;

impl<T: RealField, const PARAMETERS: usize, const OUTPUTS: usize>
    MorrisReport<T, PARAMETERS, OUTPUTS>
{
    /// Wrap populated Morris moments in the shared report shape.
    pub(crate) fn from_moments(
        effect_count: u64,
        mu: [[T; PARAMETERS]; OUTPUTS],
        mu_star: [[T; PARAMETERS]; OUTPUTS],
        sigma: [[T; PARAMETERS]; OUTPUTS],
    ) -> Self {
        Self::new(effect_count, [mu, mu_star, sigma])
    }
}

impl<T, const PARAMETERS: usize, const OUTPUTS: usize> MorrisReport<T, PARAMETERS, OUTPUTS> {
    /// Elementary effects per parameter.
    #[must_use]
    pub const fn effect_count(&self) -> u64 {
        self.sample_count()
    }

    /// Mean elementary effect per parameter for every output.
    #[must_use]
    pub const fn mu_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        self.by_output(0)
    }

    /// Mean absolute elementary effect per parameter for every output.
    #[must_use]
    pub const fn mu_star_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        self.by_output(1)
    }

    /// Elementary-effect standard deviation per parameter for every output.
    #[must_use]
    pub const fn sigma_by_output(&self) -> &[[T; PARAMETERS]; OUTPUTS] {
        self.by_output(2)
    }
}

impl<T, const PARAMETERS: usize> MorrisReport<T, PARAMETERS, 1> {
    /// Mean elementary effect for the single output.
    #[must_use]
    pub const fn mu(&self) -> &[T; PARAMETERS] {
        self.single(0)
    }

    /// Mean absolute elementary effect for the single output.
    #[must_use]
    pub const fn mu_star(&self) -> &[T; PARAMETERS] {
        self.single(1)
    }

    /// Elementary-effect standard deviation for the single output.
    #[must_use]
    pub const fn sigma(&self) -> &[T; PARAMETERS] {
        self.single(2)
    }
}
