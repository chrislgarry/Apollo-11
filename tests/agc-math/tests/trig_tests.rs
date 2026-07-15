//! Property-based tests for AGC trigonometric functions.

use agc_math::trig::*;
use proptest::prelude::*;
use std::f64::consts::PI;

proptest! {
    /// Pythagorean identity: sin²(x) + cos²(x) = 1.
    /// AGC sine/cosine return half-scaled values, so multiply by 2 first.
    #[test]
    fn pythagorean_identity(x in -0.5f64..0.5) {
        let s = agc_sine(x) * 2.0;
        let c = agc_cosine(x) * 2.0;
        let sum = s * s + c * c;
        prop_assert!(
            (sum - 1.0).abs() < 1e-3,
            "sin²({}) + cos²({}) = {} (expected 1.0)",
            x, x, sum
        );
    }

    /// Sine is an odd function: sin(-x) = -sin(x).
    #[test]
    fn sine_odd_function(x in 0.001f64..0.499) {
        let pos = agc_sine(x);
        let neg = agc_sine(-x);
        prop_assert!(
            (pos + neg).abs() < 1e-6,
            "sin({}) + sin(-{}) = {} (expected 0)",
            x, x, pos + neg
        );
    }

    /// Cosine is an even function: cos(-x) = cos(x).
    #[test]
    fn cosine_even_function(x in 0.001f64..0.499) {
        let pos = agc_cosine(x);
        let neg = agc_cosine(-x);
        prop_assert!(
            (pos - neg).abs() < 1e-6,
            "cos({}) - cos(-{}) = {} (expected 0)",
            x, x, pos - neg
        );
    }

    /// arcsin(x) + arccos(x) = pi/2 = 0.25 revolutions.
    #[test]
    fn arcsin_arccos_complementary(x in -0.95f64..0.95) {
        let sum = agc_arcsin(x) + agc_arccos(x);
        prop_assert!(
            (sum - 0.25).abs() < 1e-3,
            "arcsin({}) + arccos({}) = {} (expected 0.25)",
            x, x, sum
        );
    }

    /// sin(arcsin(x)) ≈ x for x in (-1, 1).
    /// agc_arcsin returns revolutions, agc_sine takes revolutions and returns half-scaled.
    #[test]
    fn sine_arcsin_roundtrip(x in -0.9f64..0.9) {
        let asin_rev = agc_arcsin(x);
        let roundtrip = agc_sine(asin_rev) * 2.0; // undo half-scaling
        prop_assert!(
            (roundtrip - x).abs() < 5e-3,
            "sin(arcsin({})) = {} (expected {})",
            x, roundtrip, x
        );
    }

    /// Sine matches IEEE f64 reference within AGC precision bounds.
    #[test]
    fn sine_matches_reference(x in -0.5f64..0.5) {
        let agc = agc_sine(x) * 2.0;
        let reference = (2.0 * PI * x).sin();
        prop_assert!(
            (agc - reference).abs() < 1e-4,
            "sine mismatch at x={}: agc={}, ref={}",
            x, agc, reference
        );
    }

    /// Arccos matches IEEE f64 reference within AGC precision bounds.
    #[test]
    fn arccos_matches_reference(x in -0.95f64..0.95) {
        let agc_rev = agc_arccos(x);
        let reference_rev = x.acos() / (2.0 * PI);
        prop_assert!(
            (agc_rev - reference_rev).abs() < 1e-3,
            "arccos mismatch at x={}: agc={} rev, ref={} rev",
            x, agc_rev, reference_rev
        );
    }
}
