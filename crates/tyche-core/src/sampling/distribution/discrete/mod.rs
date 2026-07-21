//! Validated finite probability-mass samplers.

mod category;
mod error;
mod importance;
mod weighted;
mod weights;

pub use category::{Categorical, CategoryCount, CategoryIndex};
pub use error::{ImportanceError, WeightError};
pub use importance::{DiscreteImportance, ImportanceSample};
pub use weighted::WeightedCategorical;
pub use weights::DiscreteWeights;
