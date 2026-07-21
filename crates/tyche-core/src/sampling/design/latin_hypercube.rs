//! Allocation-free random-access affine-permutation Latin hypercube.

use core::{marker::PhantomData, num::NonZeroU32};

use super::super::{Design, SampleIndexError};
use crate::sampling::counter::{
    Counter, LatinHypercubeJitter, LatinHypercubeOffset, LatinHypercubeStride, Seed,
    StreamAlgorithm,
};

/// A deterministic Latin hypercube with `PARAMETERS` compile-time dimensions.
///
/// Each dimension uses an affine permutation `a*i+b (mod n)` whose stride is
/// coprime with `n`. The complete design therefore places exactly one point in
/// every stratum of every dimension without storing an `n × PARAMETERS`
/// design matrix.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LatinHypercube<const PARAMETERS: usize, A> {
    seed: Seed,
    sample_count: NonZeroU32,
    strides: [u32; PARAMETERS],
    offsets: [u32; PARAMETERS],
    algorithm: PhantomData<A>,
}

impl<const PARAMETERS: usize, A: StreamAlgorithm> LatinHypercube<PARAMETERS, A> {
    /// Construct a random-access design.
    ///
    /// A non-zero sample count is carried by [`NonZeroU32`]. Bounding the
    /// count to 32 bits keeps every stratum exactly representable in `f64`. A
    /// zero-dimensional design is rejected by [`crate::ParameterSpace`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZeroU32;
    /// use tyche_core::sampling::{LatinHypercube, Seed, Design, SplitMix64};
    ///
    /// let seed = Seed::new(42);
    /// let lh = LatinHypercube::<3, SplitMix64>::new(
    ///     seed,
    ///     NonZeroU32::new(10).unwrap(),
    /// );
    /// assert_eq!(lh.sample_count(), 10);
    /// ```
    pub fn new(seed: Seed, sample_count: NonZeroU32) -> Self {
        let count = sample_count.get();
        let mut strides = [1; PARAMETERS];
        let mut offsets = [0; PARAMETERS];
        for dimension in 0..PARAMETERS {
            let coordinate = u64_from_usize(dimension);
            let candidate = u32_from_u64(
                Counter::<LatinHypercubeStride, A>::word(seed, coordinate, 0) % u64::from(count),
            );
            strides[dimension] = coprime_stride(candidate, count);
            offsets[dimension] = u32_from_u64(
                Counter::<LatinHypercubeOffset, A>::word(seed, coordinate, 0) % u64::from(count),
            );
        }
        Self {
            seed,
            sample_count,
            strides,
            offsets,
            algorithm: PhantomData,
        }
    }

    /// Stable study seed.
    pub const fn seed(&self) -> Seed {
        self.seed
    }

    /// Stratum containing a sample in one dimension.
    ///
    /// # Errors
    ///
    /// Returns [`SampleIndexError`] for an out-of-range sample or dimension.
    pub fn stratum(&self, sample: usize, dimension: usize) -> Result<usize, SampleIndexError> {
        let sample_count = usize_from_u32(self.sample_count.get());
        if sample >= sample_count || dimension >= PARAMETERS {
            return Err(SampleIndexError::new(sample, sample_count));
        }
        let Ok(sample) = u32::try_from(sample) else {
            unreachable!("invariant: validated sample index is bounded by u32 count");
        };
        let count = u64::from(self.sample_count.get());
        let product = u64::from(self.strides[dimension]) * u64::from(sample);
        let stratum = u32_from_u64((product + u64::from(self.offsets[dimension])) % count);
        Ok(usize_from_u32(stratum))
    }
}

impl<const PARAMETERS: usize, A: StreamAlgorithm> Design<PARAMETERS>
    for LatinHypercube<PARAMETERS, A>
{
    fn sample_count(&self) -> usize {
        usize_from_u32(self.sample_count.get())
    }

    fn sample_unit_into(
        &self,
        index: usize,
        output: &mut [f64; PARAMETERS],
    ) -> Result<(), SampleIndexError> {
        let sample_count = usize_from_u32(self.sample_count.get());
        if index >= sample_count {
            return Err(SampleIndexError::new(index, sample_count));
        }
        let inverse_count = 1.0 / f64::from(self.sample_count.get());
        for (dimension, coordinate) in output.iter_mut().enumerate() {
            let stratum = self.stratum(index, dimension)?;
            let Ok(stratum) = u32::try_from(stratum) else {
                unreachable!("invariant: stratum is bounded by u32 count");
            };
            let jitter = Counter::<LatinHypercubeJitter, A>::unit::<f64>(
                self.seed,
                u64_from_usize(index),
                u64_from_usize(dimension),
            );
            *coordinate = (f64::from(stratum) + jitter) * inverse_count;
        }
        Ok(())
    }
}

fn coprime_stride(candidate: u32, modulus: u32) -> u32 {
    if modulus == 1 {
        return 1;
    }
    let mut stride = candidate.max(1);
    loop {
        if greatest_common_divisor(stride, modulus) == 1 {
            return stride;
        }
        stride = if stride + 1 == modulus { 1 } else { stride + 1 };
    }
}

const fn greatest_common_divisor(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

fn u32_from_u64(value: u64) -> u32 {
    match u32::try_from(value) {
        Ok(value) => value,
        Err(_) => unreachable!("invariant: value is reduced modulo a u32 count"),
    }
}

fn usize_from_u32(value: u32) -> usize {
    match usize::try_from(value) {
        Ok(value) => value,
        Err(_) => unreachable!("invariant: Tyche requires a target with at least 32-bit usize"),
    }
}

fn u64_from_usize(value: usize) -> u64 {
    match u64::try_from(value) {
        Ok(value) => value,
        Err(_) => unreachable!("invariant: Tyche supports targets with at most 64-bit usize"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sampling::SplitMix64;

    #[test]
    fn coefficient_and_jitter_domains_do_not_reproduce_prior_aliases() {
        let seed = Seed::new(0x5459_4348_455F_444F);
        for dimension in 0..6_u64 {
            let later_dimension = dimension + 2;
            let stride =
                Counter::<LatinHypercubeStride, SplitMix64>::word(seed, later_dimension, 0);
            let offset =
                Counter::<LatinHypercubeOffset, SplitMix64>::word(seed, later_dimension, 0);
            let first_jitter =
                Counter::<LatinHypercubeJitter, SplitMix64>::word(seed, 0, dimension);
            let second_jitter =
                Counter::<LatinHypercubeJitter, SplitMix64>::word(seed, 1, dimension);

            assert_ne!(first_jitter, stride);
            assert_ne!(second_jitter, offset);
        }
    }
}
