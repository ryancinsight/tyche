//! Reproducible study example.

use core::num::NonZeroU32;
use tyche::{Ensemble, LatinHypercube, Parameter, ParameterSpace, PopulationVariance, Seed, Study};

fn main() {
    let space = ParameterSpace::new([
        Parameter::borrowed("conductivity", 0.1_f64, 1.0).expect("valid"),
        Parameter::borrowed("density", 900.0, 1_100.0).expect("valid"),
    ])
    .expect("unique");
    let design = LatinHypercube::new(
        Seed::new(0x5459_4348_455F_3031),
        NonZeroU32::new(256).expect("positive"),
    );
    let study = Study::borrowed("diffusivity proxy", space, design).expect("named");
    let responses: Vec<_> = (0..study.sample_count())
        .map(|index| {
            let sample = study.sample(index).expect("valid");
            sample.values()[0] / sample.values()[1]
        })
        .collect();
    let moments = Ensemble::new(&responses).moments().expect("non-empty");
    println!(
        "{}: {} trials, mean {:.9e}, variance {:.9e}",
        study.name(),
        moments.count(),
        moments.mean().expect("defined"),
        moments.variance::<PopulationVariance>().expect("defined")
    );
}
