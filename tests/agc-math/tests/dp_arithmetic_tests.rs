//! Property-based tests for double-precision arithmetic.

use agc_math::arithmetic::*;
use agc_math::ones_complement::*;
use proptest::prelude::*;

/// Strategy to generate DP values as fractions in (-1, 1).
/// Avoids values too close to ±1 to prevent overflow in tests.
fn dp_fraction() -> impl Strategy<Value = DpWord> {
    (-9000i16..=9000).prop_map(|v| {
        DpWord::new(AgcWord::from_i16(v), AgcWord::POS_ZERO)
    })
}

/// Strategy for small DP fractions (safe for multiply without overflow).
fn small_dp() -> impl Strategy<Value = DpWord> {
    (-8000i16..=8000).prop_map(|v| {
        DpWord::new(AgcWord::from_i16(v), AgcWord::POS_ZERO)
    })
}

proptest! {
    /// DAD is commutative: a + b = b + a.
    #[test]
    fn dad_commutative(a in dp_fraction(), b in dp_fraction()) {
        let (r1, _) = dad(a, b);
        let (r2, _) = dad(b, a);
        prop_assert_eq!(r1, r2);
    }

    /// DAD with zero is identity: a + 0 = a.
    #[test]
    fn dad_identity(a in dp_fraction()) {
        let (result, _) = dad(a, DpWord::ZERO);
        prop_assert_eq!(result, a);
    }

    /// DSU(a, a) ≈ 0 (self-subtraction).
    #[test]
    fn dsu_self_is_zero(a in dp_fraction()) {
        let (result, _) = dsu(a, a);
        prop_assert!(
            result.to_f64().abs() < 1e-6,
            "DSU({:?}, self) = {:?} ≈ {}",
            a, result, result.to_f64()
        );
    }

    /// DMP is commutative: a * b = b * a.
    #[test]
    fn dmp_commutative(a in small_dp(), b in small_dp()) {
        let r1 = dmp(a, b);
        let r2 = dmp(b, a);
        // Allow 1 LSB difference due to truncation order
        prop_assert!(
            (r1.to_f64() - r2.to_f64()).abs() < 1e-6,
            "DMP({:?}, {:?}) = {} vs {}",
            a, b, r1.to_f64(), r2.to_f64()
        );
    }

    /// DMP by zero gives zero.
    #[test]
    fn dmp_by_zero(a in small_dp()) {
        let result = dmp(a, DpWord::ZERO);
        prop_assert!(
            result.to_f64().abs() < 1e-8,
            "DMP({:?}, 0) = {} (expected 0)",
            a, result.to_f64()
        );
    }

    /// DSQ(a) = DMP(a, a).
    #[test]
    fn dsq_equals_dmp_self(a in small_dp()) {
        let sq = dsq(a);
        let mp = dmp(a, a);
        prop_assert_eq!(sq, mp);
    }

    /// DSQ result is non-negative.
    #[test]
    fn dsq_non_negative(a in small_dp()) {
        let result = dsq(a);
        prop_assert!(
            result.to_f64() >= -1e-8,
            "DSQ({:?}) = {} (expected >= 0)",
            a, result.to_f64()
        );
    }

    /// DAD then DSU is identity: (a + b) - b ≈ a.
    #[test]
    fn dad_dsu_roundtrip(
        a_val in -4000i16..=4000,
        b_val in -4000i16..=4000,
    ) {
        let a = DpWord::new(AgcWord::from_i16(a_val), AgcWord::POS_ZERO);
        let b = DpWord::new(AgcWord::from_i16(b_val), AgcWord::POS_ZERO);
        let (sum, ovf1) = dad(a, b);
        if !ovf1 {
            let (result, _) = dsu(sum, b);
            prop_assert!(
                (result.to_f64() - a.to_f64()).abs() < 1e-6,
                "(a + b) - b: expected {}, got {}",
                a.to_f64(), result.to_f64()
            );
        }
    }
}

#[test]
fn test_dmp_known_values() {
    // 0.5 * 0.5 = 0.25
    let half = DpWord::from_f64(0.5);
    let result = dmp(half, half);
    assert!(
        (result.to_f64() - 0.25).abs() < 1e-5,
        "0.5 * 0.5 = {} (expected 0.25)",
        result.to_f64()
    );

    // 0.25 * 0.5 = 0.125
    let quarter = DpWord::from_f64(0.25);
    let result = dmp(quarter, half);
    assert!(
        (result.to_f64() - 0.125).abs() < 1e-5,
        "0.25 * 0.5 = {} (expected 0.125)",
        result.to_f64()
    );
}

#[test]
fn test_dmp_negative() {
    let pos = DpWord::from_f64(0.5);
    let neg = DpWord::from_f64(-0.5);

    let pp = dmp(pos, pos);
    assert!(pp.to_f64() > 0.0, "pos * pos should be positive");

    let pn = dmp(pos, neg);
    assert!(pn.to_f64() < 0.0, "pos * neg should be negative");

    let nn = dmp(neg, neg);
    assert!(nn.to_f64() > 0.0, "neg * neg should be positive");
}
