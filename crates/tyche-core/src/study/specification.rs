//! Study identity, parameter space, and design composition.

use alloc::borrow::Cow;

use eunomia::RealField;

use crate::design::ParameterSpace;
use crate::sampling::Design;

use super::{Sample, StudyError};

/// A named reproducible study over a statically dispatched design.
#[derive(Debug, Clone, PartialEq)]
pub struct Study<'a, T, D, const PARAMETERS: usize> {
    name: Cow<'a, str>,
    space: ParameterSpace<'a, T, PARAMETERS>,
    design: D,
}

impl<'a, T, D, const PARAMETERS: usize> Study<'a, T, D, PARAMETERS>
where
    T: RealField,
    D: Design<PARAMETERS>,
{
    /// Construct a borrowed-name study.
    ///
    /// # Errors
    ///
    /// Returns [`StudyError::EmptyName`] when the name is empty.
    pub fn borrowed(
        name: &'a str,
        space: ParameterSpace<'a, T, PARAMETERS>,
        design: D,
    ) -> Result<Self, StudyError> {
        Self::new(Cow::Borrowed(name), space, design)
    }

    /// Construct an owned-name study.
    ///
    /// # Errors
    ///
    /// Returns [`StudyError::EmptyName`] when the name is empty.
    pub fn owned(
        name: alloc::string::String,
        space: ParameterSpace<'a, T, PARAMETERS>,
        design: D,
    ) -> Result<Self, StudyError> {
        Self::new(Cow::Owned(name), space, design)
    }

    fn new(
        name: Cow<'a, str>,
        space: ParameterSpace<'a, T, PARAMETERS>,
        design: D,
    ) -> Result<Self, StudyError> {
        if name.is_empty() {
            return Err(StudyError::EmptyName);
        }
        Ok(Self {
            name,
            space,
            design,
        })
    }

    /// Stable study name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether the study name is borrowed.
    #[must_use]
    pub const fn is_name_borrowed(&self) -> bool {
        matches!(self.name, Cow::Borrowed(_))
    }

    /// Parameter-space definition.
    #[must_use]
    pub const fn space(&self) -> &ParameterSpace<'a, T, PARAMETERS> {
        &self.space
    }

    /// Experimental design.
    #[must_use]
    pub const fn design(&self) -> &D {
        &self.design
    }

    /// Number of trials.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.design.sample_count()
    }

    /// Generate one sample directly into a fixed array.
    ///
    /// Returns `None` when `index` is outside the design.
    #[must_use]
    pub fn sample(&self, index: usize) -> Option<Sample<T, PARAMETERS>> {
        let mut unit = [0.0; PARAMETERS];
        self.design.sample_unit_into(index, &mut unit).ok()?;
        let mut values = [T::ZERO; PARAMETERS];
        self.space.map_unit_into(&unit, &mut values);
        Some(Sample::new(index, values))
    }
}
