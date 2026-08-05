//! Online moment accumulation with Welford-Chan incremental statistics.
//!
//! [`Moments`] accumulates mean and variance online — each `update` call
//! takes O(1) time and O(1) space.  The algorithm is numerically stable
//! against catastrophic cancellation in the variance calculation.

use tyche_core::{Moments, PopulationVariance, SampleVariance};

fn main() {
    let mut m = Moments::<f64>::new();
    let data = [2.0_f64, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    for &value in &data {
        m.update(value);
    }

    let mean = m.mean().expect("non-empty");
    let sample_var = m.variance::<SampleVariance>().expect("≥2 samples");
    let pop_var = m.variance::<PopulationVariance>().expect("≥1 sample");

    println!("n    = {}", data.len());
    println!("mean = {mean}"); // 5.0
    println!("sample variance = {sample_var:.4}"); // 4.5714…
    println!("population variance = {pop_var:.4}"); // 4.0

    assert!((mean - 5.0).abs() < 1e-10);
    assert!((sample_var - 4.571_428_571).abs() < 1e-6);
    assert!((pop_var - 4.0).abs() < 1e-10);

    // ── Merge two independent moment accumulators ──
    let mut m1 = Moments::<f64>::new();
    let mut m2 = Moments::<f64>::new();
    for &v in &data[..4] {
        m1.update(v);
    }
    for &v in &data[4..] {
        m2.update(v);
    }
    m1.merge(m2);
    let merged_mean = m1.mean().expect("merged");
    assert!(
        (merged_mean - mean).abs() < 1e-10,
        "merged mean must equal full-batch mean"
    );
    println!("merged mean = {merged_mean}");

    println!("all moments assertions passed");
}
