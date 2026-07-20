//! Compile-time-dimensional parameter-space composition.

use core::fmt;

use eunomia::RealField;

use super::Parameter;

/// A fixed-dimensional parameter space.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterSpace<'a, T, const PARAMETERS: usize> {
    parameters: [Parameter<'a, T>; PARAMETERS],
}

impl<'a, T: RealField, const PARAMETERS: usize> ParameterSpace<'a, T, PARAMETERS> {
    /// Construct a parameter space.
    ///
    /// # Errors
    ///
    /// Returns [`SpaceError::ZeroDimensions`] when `PARAMETERS == 0`, or
    /// [`SpaceError::DuplicateName`] when names are not unique.
    pub fn new(parameters: [Parameter<'a, T>; PARAMETERS]) -> Result<Self, SpaceError> {
        if PARAMETERS == 0 {
            return Err(SpaceError::ZeroDimensions);
        }
        for left in 0..PARAMETERS {
            for right in (left + 1)..PARAMETERS {
                if parameters[left].name() == parameters[right].name() {
                    return Err(SpaceError::DuplicateName {
                        first: left,
                        second: right,
                    });
                }
            }
        }
        Ok(Self { parameters })
    }

    /// Borrow the parameter definitions without copying them.
    #[must_use]
    pub const fn parameters(&self) -> &[Parameter<'a, T>; PARAMETERS] {
        &self.parameters
    }

    /// Map a unit-hypercube point directly into caller-provided storage.
    pub fn map_unit_into(&self, unit: &[f64; PARAMETERS], output: &mut [T; PARAMETERS]) {
        for dimension in 0..PARAMETERS {
            output[dimension] = self.parameters[dimension].map_unit(unit[dimension]);
        }
    }
}

/// Parameter-space construction failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpaceError {
    /// A zero-dimensional study has no sampleable parameter.
    ZeroDimensions,
    /// Two parameters share the same name.
    DuplicateName {
        /// Earlier parameter index.
        first: usize,
        /// Later parameter index.
        second: usize,
    },
}

impl fmt::Display for SpaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDimensions => formatter.write_str("parameter space must have a dimension"),
            Self::DuplicateName { first, second } => {
                write!(
                    formatter,
                    "parameter names at indices {first} and {second} are identical"
                )
            }
        }
    }
}

impl core::error::Error for SpaceError {}
