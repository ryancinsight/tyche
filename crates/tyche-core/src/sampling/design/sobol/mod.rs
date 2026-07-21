//! Random-access Sobol designs with explicit sequence ranges and scrambling.
//!
//! For point index `n`, let `g(n) = n ^ (n >> 1)` be its Gray code and
//! `v[d][j]` the direction number for dimension `d` and bit `j`. The canonical
//! kernel evaluates
//!
//! `x[n][d] = XOR { v[d][j] | bit j of g(n) is set } / 2^32`.
//!
//! This is the random-access form of Bratley and Fox's recurrence. Consecutive
//! Gray codes differ at exactly the bit selected by the trailing-zero count of
//! the new sequence index, so the XOR above changes by the same single
//! direction number as the sequential recurrence. Induction from `x[0] = 0`
//! proves equivalence for every 32-bit index. The implementation is checked
//! against an independent sequential oracle in the integration tests.
//!
//! Direction parameters follow Joe and Kuo's notation and the first three
//! dimensions used by Algorithm 659:
//! <https://web.maths.unsw.edu.au/~fkuo/sobol/joe-kuo-notes.pdf>.

mod direction;
mod error;
mod fixed;
mod kernel;
mod policy;
mod range;
mod runtime;

pub use error::{RuntimeSampleError, SobolDimensionError, SobolRangeError};
pub use fixed::Sobol;
pub use policy::{DigitalShift, SobolScramble, Unscrambled};
pub use range::{SobolDimensions, SobolRange};
pub use runtime::RuntimeSobol;
