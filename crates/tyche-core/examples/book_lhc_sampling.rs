//! Parameter spaces and Latin hypercube sampling.
//!
//! [`LatinHypercube`] generates deterministic, random-access samples in a
//! unit hypercube.  Each sample index maps to exactly one point; no two
//! indices share the same stratum in any dimension.
//!
//! [`Parameter::map_unit`] scales a unit-interval value to the parameter
//! bounds, completing the unit-hypercube → physical-space mapping.

use core::num::NonZeroU32;
use tyche_core::{Design, LatinHypercube, Parameter, ParameterSpace, Seed, SplitMix64};

fn main() {
    // ── Two-parameter space: pressure [0.1, 1.0] MPa, temperature [300, 400] K ──
    let pressure =
        Parameter::borrowed("pressure_MPa", 0.1_f64, 1.0).expect("finite, ordered bounds");
    let temperature =
        Parameter::borrowed("temperature_K", 300.0_f64, 400.0).expect("finite, ordered bounds");
    let space = ParameterSpace::new([pressure, temperature]).expect("unique names");

    // ── LHC design: 8 samples in 2 dimensions ──
    let seed = Seed::new(7);
    let lh = LatinHypercube::<2, SplitMix64>::new(seed, NonZeroU32::new(8).expect("nonzero"));
    assert_eq!(lh.sample_count(), 8);

    println!("{:>10} {:>14} {:>14}", "index", "p (MPa)", "T (K)");
    println!("{}", "-".repeat(42));
    for i in 0..lh.sample_count() {
        let mut unit = [0.0_f64; 2];
        lh.sample_unit_into(i, &mut unit).expect("valid index");

        // Map unit hypercube to physical parameter bounds.
        let p = space.parameters()[0].map_unit(unit[0]);
        let t = space.parameters()[1].map_unit(unit[1]);
        println!("{i:>10} {p:>14.4} {t:>14.2}");

        // Each sample must lie inside the parameter bounds.
        assert!((0.1..=1.0).contains(&p));
        assert!((300.0..=400.0).contains(&t));
    }

    println!("all LHC assertions passed");
}
