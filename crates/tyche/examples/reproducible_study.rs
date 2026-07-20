//! Reproducible Latin-hypercube study with ordered ensemble moments.

use core::num::NonZeroUsize;

use tyche::{Ensemble, LatinHypercube, Parameter, ParameterSpace, PopulationVariance, Seed, Study};

fn main() {
    let space = ParameterSpace::new([
        Parameter::borrowed("conductivity", 0.1_f64, 1.0).expect("valid"),
        Parameter::borrowed("density", 900.0, 1_100.0).expect("valid"),
    ])
    .expect("unique parameter names");
    let design = LatinHypercube::new(
        Seed::new(0x5459_4348_455F_3031),
        NonZeroUsize::new(256).expect("constant is positive"),
    );
    let study = Study::borrowed("diffusivity proxy", space, design).expect("named");

    let responses: Vec<_> = (0..study.sample_count())
        .map(|index| {
            let sample = study.sample(index).expect("valid index");
            sample.values()[0] / sample.values()[1]
        })
        .collect();
    let moments = Ensemble::new(&responses).moments().expect("non-empty");

    println!("study: {}", study.name());
    println!("trials: {}", moments.count());
    println!("mean: {:.9e}", moments.mean().expect("non-empty"));
    println!(
        "population variance: {:.9e}",
        moments.variance::<PopulationVariance>().expect("non-empty")
    );
}
