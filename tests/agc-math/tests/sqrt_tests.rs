//! Property-based tests for AGC square root.

use agc_math::arithmetic::dsq;
use agc_math::ones_complement::DpWord;
use agc_math::sqrt::agc_sqrt;
use proptest::prelude::*;

proptest! {
    /// sqrt(x^2) ≈ |x| for positive values.
    #[test]
    fn sqrt_of_square_is_abs(val in 0.01f64..0.9) {
        let x = DpWord::from_f64(val);
        let x_sq = dsq(x);
        let result = agc_sqrt(x_sq);
        prop_assert!(
            (result.to_f64() - val).abs() < 1e-3,
            "sqrt({}^2) = {}, expected {}",
            val, result.to_f64(), val
        );
    }

    /// sqrt result is non-negative.
    #[test]
    fn sqrt_non_negative(val in 0.0f64..0.99) {
        let x = DpWord::from_f64(val);
        let result = agc_sqrt(x);
        prop_assert!(
            result.to_f64() >= -1e-10,
            "sqrt({}) = {} (expected >= 0)",
            val, result.to_f64()
        );
    }

    /// sqrt matches f64 reference.
    #[test]
    fn sqrt_matches_reference(val in 0.001f64..0.99) {
        let x = DpWord::from_f64(val);
        let result = agc_sqrt(x);
        let reference = val.sqrt();
        prop_assert!(
            (result.to_f64() - reference).abs() < 1e-3,
            "sqrt({}): agc={}, ref={}",
            val, result.to_f64(), reference
        );
    }
}

#[test]
fn test_sqrt_zero() {
    let result = agc_sqrt(DpWord::ZERO);
    assert!(result.to_f64().abs() < 1e-10);
}

#[test]
fn test_sqrt_quarter() {
    let result = agc_sqrt(DpWord::from_f64(0.25));
    assert!(
        (result.to_f64() - 0.5).abs() < 1e-4,
        "sqrt(0.25) = {}",
        result.to_f64()
    );
}
