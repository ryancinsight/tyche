//! Stateless probability-distribution samplers.

mod discrete;
mod normal;

pub use discrete::{
    Categorical, CategoryCount, CategoryIndex, DiscreteImportance, DiscreteWeights,
    ImportanceError, ImportanceSample, WeightError, WeightedCategorical,
};
pub use normal::StandardNormal;
