//! Borrowed Moirai execution adapter for Tyche studies.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod dispatch;
mod error;

pub use dispatch::MoiraiDispatch;
pub use error::DispatchError;
