//! One named, bounded experimental parameter.

use alloc::borrow::Cow;
use core::fmt;

use eunomia::RealField;

/// A named finite parameter interval.
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter<'a, T> {
    name: Cow<'a, str>,
    lower: T,
    upper: T,
}

impl<'a, T: RealField> Parameter<'a, T> {
    /// Construct a borrowed-name parameter.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidParameter`] unless both bounds are finite and
    /// `lower < upper`.
    pub fn borrowed(name: &'a str, lower: T, upper: T) -> Result<Self, InvalidParameter<T>> {
        Self::new(Cow::Borrowed(name), lower, upper)
    }

    /// Construct an owned-name parameter.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidParameter`] unless both bounds are finite and
    /// `lower < upper`.
    pub fn owned(
        name: alloc::string::String,
        lower: T,
        upper: T,
    ) -> Result<Self, InvalidParameter<T>> {
        Self::new(Cow::Owned(name), lower, upper)
    }

    fn new(name: Cow<'a, str>, lower: T, upper: T) -> Result<Self, InvalidParameter<T>> {
        if name.is_empty() {
            return Err(InvalidParameter::EmptyName);
        }
        if !lower.is_finite() || !upper.is_finite() {
            return Err(InvalidParameter::NonFiniteBounds { lower, upper });
        }
        if lower >= upper {
            return Err(InvalidParameter::UnorderedBounds { lower, upper });
        }
        Ok(Self { name, lower, upper })
    }

    /// Parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether the name is borrowed.
    #[must_use]
    pub const fn is_name_borrowed(&self) -> bool {
        matches!(self.name, Cow::Borrowed(_))
    }

    /// Inclusive lower bound.
    #[must_use]
    pub const fn lower(&self) -> T {
        self.lower
    }

    /// Exclusive sampling upper bound.
    #[must_use]
    pub const fn upper(&self) -> T {
        self.upper
    }

    /// Map a normalized coordinate in `[0, 1)` into the parameter interval.
    #[must_use]
    pub fn map_unit(&self, unit: f64) -> T {
        let coordinate = T::from_f64(unit);
        self.lower + coordinate * (self.upper - self.lower)
    }
}

/// Parameter construction failure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InvalidParameter<T> {
    /// The parameter has no stable identity.
    EmptyName,
    /// At least one bound is NaN or infinite.
    NonFiniteBounds {
        /// Supplied lower bound.
        lower: T,
        /// Supplied upper bound.
        upper: T,
    },
    /// The lower bound is not strictly below the upper bound.
    UnorderedBounds {
        /// Supplied lower bound.
        lower: T,
        /// Supplied upper bound.
        upper: T,
    },
}

impl<T: fmt::Debug> fmt::Display for InvalidParameter<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("parameter name must not be empty"),
            Self::NonFiniteBounds { lower, upper } => {
                write!(
                    formatter,
                    "parameter bounds must be finite: [{lower:?}, {upper:?})"
                )
            }
            Self::UnorderedBounds { lower, upper } => {
                write!(
                    formatter,
                    "parameter bounds must satisfy lower < upper: [{lower:?}, {upper:?})"
                )
            }
        }
    }
}

impl<T: fmt::Debug + 'static> core::error::Error for InvalidParameter<T> {}
