//! Bagging over the provider's deterministic bootstrap resamples.

use alloc::vec::Vec;
use core::{fmt, marker::PhantomData, num::NonZeroU32};

use eunomia::RealField;

use super::TrainableModel;
use crate::sampling::{Bootstrap, Seed, StreamAlgorithm};
use crate::statistics::Moments;

/// A bagging design: member `r` fits the resample at replicate `r` of the
/// wrapped [`Bootstrap`].
///
/// The draw schedule is the provider's and replays under `A::VERSION`; bagging
/// adds no random stream.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bagging<A> {
    bootstrap: Bootstrap<A>,
    members: NonZeroU32,
}

impl<A: StreamAlgorithm> Bagging<A> {
    /// Construct a design of `members` fits over `bootstrap` resamples.
    pub const fn new(bootstrap: Bootstrap<A>, members: NonZeroU32) -> Self {
        Self { bootstrap, members }
    }

    /// Fit one model per resample and retain every trained state.
    ///
    /// # Errors
    ///
    /// Returns [`BaggingError::PopulationLength`] when `observations` does not
    /// match the bootstrap population, [`BaggingError::ResampleAllocation`]
    /// when the resample buffer cannot be allocated, else the first member's
    /// failure.
    ///
    /// # Panics
    ///
    /// Only if the `fill_into` length invariant breaks: the resample buffer is
    /// resized to exactly [`Bootstrap::resample_size`] before every fill.
    pub fn fit<'model, M, T>(
        self,
        model: &'model M,
        seed: Seed,
        observations: &[M::Observation],
    ) -> Result<BaggedModel<'model, M, T>, BaggingError<M::Error>>
    where
        M: TrainableModel<T>,
    {
        let expected = self.bootstrap.population_size();
        if observations.len() != expected {
            return Err(BaggingError::PopulationLength {
                expected,
                actual: observations.len(),
            });
        }
        let resample_size = self.bootstrap.resample_size();
        let mut resample = Vec::new();
        resample
            .try_reserve_exact(resample_size)
            .map_err(|_| BaggingError::ResampleAllocation { resample_size })?;
        resample.resize(resample_size, 0);
        let fitted = (0..self.members.get())
            .map(|replicate| {
                self.bootstrap
                    .fill_into(seed, u64::from(replicate), &mut resample)
                    .expect("invariant: the buffer length equals the resample size");
                model
                    .fit(observations, &resample)
                    .map_err(BaggingError::Model)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(BaggedModel {
            model,
            fitted,
            scalar: PhantomData,
        })
    }
}

/// Retained member states that predict by their mean, without refitting.
#[must_use]
pub struct BaggedModel<'model, M: TrainableModel<T>, T> {
    model: &'model M,
    fitted: Vec<M::Fitted>,
    scalar: PhantomData<fn() -> T>,
}

impl<M: TrainableModel<T>, T: RealField> BaggedModel<'_, M, T> {
    /// Trained state of each member in replicate order.
    #[must_use]
    pub fn fitted_members(&self) -> &[M::Fitted] {
        &self.fitted
    }

    /// Mean of the members' predictions in replicate order.
    ///
    /// # Errors
    ///
    /// Returns the first member's prediction failure.
    ///
    /// # Panics
    ///
    /// Never in practice: a bagged model holds at least one member.
    pub fn predict(&self, input: &M::Input) -> Result<T, M::Error> {
        let mut moments = Moments::new();
        for fitted in &self.fitted {
            moments.update(self.model.predict(fitted, input)?);
        }
        Ok(moments
            .mean()
            .expect("invariant: a bagged model holds at least one member"))
    }
}

/// A bagging fit failure.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BaggingError<E> {
    /// The observations do not match the bootstrap population.
    PopulationLength {
        /// Population size the bootstrap draws from.
        expected: usize,
        /// Supplied observation count.
        actual: usize,
    },
    /// The resample buffer of `resample_size` indices cannot be allocated.
    ResampleAllocation {
        /// Index count of the requested resample.
        resample_size: usize,
    },
    /// A member's fit failed.
    Model(E),
}

impl<E> fmt::Display for BaggingError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PopulationLength { expected, actual } => write!(
                formatter,
                "bagging received {actual} observations for a population of {expected}"
            ),
            Self::ResampleAllocation { resample_size } => write!(
                formatter,
                "bagging cannot allocate a resample of {resample_size} indices"
            ),
            Self::Model(_) => formatter.write_str("bagging member fit failed"),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for BaggingError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::PopulationLength { .. } | Self::ResampleAllocation { .. } => None,
            Self::Model(source) => Some(source),
        }
    }
}
