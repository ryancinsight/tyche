//! Allocation-free random-access affine-permutation Latin hypercube.

use core::num::NonZeroUsize;

use super::{Design, SampleIndexError, Seed, SplitMix64};

/// A deterministic Latin hypercube with `PARAMETERS` compile-time dimensions.
///
/// Each dimension uses an affine permutation `a*i+b (mod n)` whose stride is
/// coprime with `n`. The complete design therefore places exactly one point in
/// every stratum of every dimension without storing an `n × PARAMETERS`
/// design matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LatinHypercube<const PARAMETERS: usize> {
    seed: Seed,
    sample_count: NonZeroUsize,
    strides: [usize; PARAMETERS],
    offsets: [usize; PARAMETERS],
}

impl<const PARAMETERS: usize> LatinHypercube<PARAMETERS> {
    /// Construct a random-access design.
    ///
    /// A non-zero sample count is carried by [`NonZeroUsize`]. A
    /// zero-dimensional design is rejected by [`crate::ParameterSpace`].
    #[must_use]
    pub fn new(seed: Seed, sample_count: NonZeroUsize) -> Self {
        let count = sample_count.get();
        let mut strides = [1; PARAMETERS];
        let mut offsets = [0; PARAMETERS];
        for dimension in 0..PARAMETERS {
            let stream = dimension as u64;
            let candidate = (SplitMix64::word(seed, 0, stream) as usize) % count;
            strides[dimension] = coprime_stride(candidate, count);
            offsets[dimension] = (SplitMix64::word(seed, 1, stream) as usize) % count;
        }
        Self {
            seed,
            sample_count,
            strides,
            offsets,
        }
    }

    /// Stable study seed.
    #[must_use]
    pub const fn seed(&self) -> Seed {
        self.seed
    }

    /// Stratum containing a sample in one dimension.
    ///
    /// # Errors
    ///
    /// Returns [`SampleIndexError`] for an out-of-range sample or dimension.
    pub fn stratum(&self, sample: usize, dimension: usize) -> Result<usize, SampleIndexError> {
        if sample >= self.sample_count.get() || dimension >= PARAMETERS {
            return Err(SampleIndexError::new(sample, self.sample_count.get()));
        }
        let count = self.sample_count.get() as u128;
        let product = (self.strides[dimension] as u128) * (sample as u128);
        Ok(((product + self.offsets[dimension] as u128) % count) as usize)
    }
}

impl<const PARAMETERS: usize> Design<PARAMETERS> for LatinHypercube<PARAMETERS> {
    fn sample_count(&self) -> usize {
        self.sample_count.get()
    }

    fn sample_unit_into(
        &self,
        index: usize,
        output: &mut [f64; PARAMETERS],
    ) -> Result<(), SampleIndexError> {
        if index >= self.sample_count.get() {
            return Err(SampleIndexError::new(index, self.sample_count.get()));
        }
        let inverse_count = 1.0 / self.sample_count.get() as f64;
        for (dimension, coordinate) in output.iter_mut().enumerate() {
            let stratum = self.stratum(index, dimension)?;
            let jitter = SplitMix64::unit(self.seed, index as u64, dimension as u64 + 2);
            *coordinate = (stratum as f64 + jitter) * inverse_count;
        }
        Ok(())
    }
}

fn coprime_stride(candidate: usize, modulus: usize) -> usize {
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

const fn greatest_common_divisor(mut left: usize, mut right: usize) -> usize {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}
