//! Validated experimental parameters and const-generic parameter spaces.

mod parameter;
mod space;

pub use parameter::{InvalidParameter, Parameter};
pub use space::{ParameterSpace, SpaceError};
