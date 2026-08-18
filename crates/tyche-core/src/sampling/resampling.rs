//! Deterministic resampling designs.

use core::{fmt, marker::PhantomData, num::NonZeroU64, num::NonZeroUsize};

use super::counter::{BootstrapIndex, Seed, StreamAlgorithm, bounded_integer};

/// A validated bootstrap index design over a finite population.
///
/// Each logical address `(seed, replicate, draw)` maps to one population index
/// by exact multiply-high rejection. The design stores only its two validated
/// sizes and a zero-sized algorithm policy; repeated random-access draws and
/// caller-owned fills allocate nothing.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bootstrap<A> {
    population_size: NonZeroUsize,
    resample_size: NonZeroUsize,
    algorithm: PhantomData<A>,
}

impl<A: StreamAlgorithm> Bootstrap<A> {
    /// Construct a bootstrap design.
    ///
    /// `population_size` is the number of source observations and
    /// `resample_size` is the number of indices in each resample. Both must be
    /// non-zero and representable by the counter's `u64` address space.
    ///
    /// # Errors
    ///
    /// Returns [`BootstrapError`] when either size is zero or exceeds the
    /// counter's addressable range.
    pub fn new(population_size: usize, resample_size: usize) -> Result<Self, BootstrapError> {
        let population_size =
            NonZeroUsize::new(population_size).ok_or(BootstrapError::EmptyPopulation)?;
        let resample_size =
            NonZeroUsize::new(resample_size).ok_or(BootstrapError::EmptyResample)?;
        if u64::try_from(population_size.get()).is_err() {
            return Err(BootstrapError::PopulationTooLarge {
                population_size: population_size.get(),
            });
        }
        if u64::try_from(resample_size.get()).is_err() {
            return Err(BootstrapError::ResampleTooLarge {
                resample_size: resample_size.get(),
            });
        }
        Ok(Self {
            population_size,
            resample_size,
            algorithm: PhantomData,
        })
    }

    /// Number of source observations.
    #[must_use]
    pub const fn population_size(self) -> usize {
        self.population_size.get()
    }

    /// Number of indices in each resample.
    #[must_use]
    pub const fn resample_size(self) -> usize {
        self.resample_size.get()
    }

    /// Return the population index at a stable logical address.
    ///
    /// `replicate` identifies the resample and `draw` identifies an output
    /// position within that resample. The mapping is random-access: callers
    /// may evaluate or repeat addresses in any order without mutable RNG state.
    ///
    /// # Panics
    ///
    /// Panics only if an internal conversion invariant established by
    /// [`Self::new`] is violated.
    #[must_use]
    pub fn at(self, seed: Seed, replicate: u64, draw: u64) -> usize {
        let bound = NonZeroU64::new(
            u64::try_from(self.population_size.get())
                .expect("invariant: constructor validates the population bound"),
        )
        .expect("invariant: constructor validates a non-zero population");
        usize::try_from(bounded_integer::<BootstrapIndex, A>(
            seed, replicate, draw, bound,
        ))
        .expect("invariant: population index fits usize")
    }

    /// Fill one caller-owned resample without allocating.
    ///
    /// The output length must equal [`Self::resample_size`]. The output is not
    /// modified when its length is invalid.
    ///
    /// # Errors
    ///
    /// Returns [`BootstrapError::OutputLength`] when `output` has the wrong
    /// length.
    ///
    /// # Panics
    ///
    /// Panics only if an internal conversion invariant established by
    /// [`Self::new`] is violated.
    pub fn fill_into(
        self,
        seed: Seed,
        replicate: u64,
        output: &mut [usize],
    ) -> Result<(), BootstrapError> {
        if output.len() != self.resample_size() {
            return Err(BootstrapError::OutputLength {
                expected: self.resample_size(),
                actual: output.len(),
            });
        }
        for (draw, slot) in output.iter_mut().enumerate() {
            *slot = self.at(
                seed,
                replicate,
                u64::try_from(draw).expect("invariant: constructor validates resample size"),
            );
        }
        Ok(())
    }
}

/// A bootstrap design size or output contract failure.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BootstrapError {
    /// The source population contains no observations.
    EmptyPopulation,
    /// A resample contains no output positions.
    EmptyResample,
    /// The population cannot be addressed by Tyche's `u64` counter.
    PopulationTooLarge {
        /// Rejected population size.
        population_size: usize,
    },
    /// The resample cannot be addressed by Tyche's `u64` counter.
    ResampleTooLarge {
        /// Rejected resample size.
        resample_size: usize,
    },
    /// Caller-owned output has the wrong length.
    OutputLength {
        /// Required output length.
        expected: usize,
        /// Supplied output length.
        actual: usize,
    },
}

impl fmt::Display for BootstrapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPopulation => formatter.write_str("bootstrap population is empty"),
            Self::EmptyResample => formatter.write_str("bootstrap resample is empty"),
            Self::PopulationTooLarge { population_size } => write!(
                formatter,
                "bootstrap population size {population_size} exceeds the u64 counter range"
            ),
            Self::ResampleTooLarge { resample_size } => write!(
                formatter,
                "bootstrap resample size {resample_size} exceeds the u64 counter range"
            ),
            Self::OutputLength { expected, actual } => write!(
                formatter,
                "bootstrap output length {actual} does not match required length {expected}"
            ),
        }
    }
}

impl core::error::Error for BootstrapError {}
