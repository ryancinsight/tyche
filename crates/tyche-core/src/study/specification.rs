//! Study identity, parameter space, and design composition.

use super::{Sample, StudyError};
use crate::design::ParameterSpace;
use crate::sampling::{Design, SampleIndexError};
use alloc::borrow::Cow;
use eunomia::RealField;

/// A named reproducible study over a statically dispatched design.
#[must_use]
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
    /// # Examples
    ///
    /// ```
    /// use tyche_core::design::{Parameter, ParameterSpace};
    /// use tyche_core::sampling::{Seed, LatinHypercube};
    /// use tyche_core::study::Study;
    /// use std::num::NonZeroU32;
    ///
    /// let x = Parameter::borrowed("x", 0.0_f64, 1.0).unwrap();
    /// let space = ParameterSpace::new([x]).unwrap();
    /// let design = LatinHypercube::<1>::new(Seed::new(0), NonZeroU32::new(5).unwrap());
    /// let study = Study::borrowed("my-study", space, design).unwrap();
    /// assert_eq!(study.name(), "my-study");
    /// ```
    ///
    /// # Errors
    ///
    /// Rejects an empty name.
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
    /// Rejects an empty name.
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

    /// Stable name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether the name is borrowed.
    #[must_use]
    pub const fn is_name_borrowed(&self) -> bool {
        matches!(self.name, Cow::Borrowed(_))
    }

    /// Parameter space.
    pub const fn space(&self) -> &ParameterSpace<'a, T, PARAMETERS> {
        &self.space
    }

    /// Design.
    #[must_use]
    pub const fn design(&self) -> &D {
        &self.design
    }

    /// Trial count.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.design.sample_count()
    }

    /// Generate one fixed-array sample.
    ///
    /// # Errors
    ///
    /// Returns [`SampleIndexError`] when `index` lies outside the design.
    pub fn sample(&self, index: usize) -> Result<Sample<T, PARAMETERS>, SampleIndexError> {
        let mut unit = [0.0; PARAMETERS];
        self.design.sample_unit_into(index, &mut unit)?;
        let mut values = [T::ZERO; PARAMETERS];
        self.space.map_unit_into(&unit, &mut values);
        Ok(Sample::new(index, values))
    }
}
