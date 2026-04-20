//! Property-based tests for AGC vector operations.

use agc_math::ones_complement::DpWord;
use agc_math::vector::*;
use proptest::prelude::*;

/// Strategy for small AGC vectors (components in safe multiply range).
fn small_vector() -> impl Strategy<Value = AgcVector> {
    (-0.4f64..0.4, -0.4f64..0.4, -0.4f64..0.4)
        .prop_map(|(x, y, z)| AgcVector::from_f64(x, y, z))
}

fn approx_eq_f64(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

proptest! {
    /// DOT is commutative: a.b = b.a
    #[test]
    fn dot_commutative(a in small_vector(), b in small_vector()) {
        let ab = dot(a, b).to_f64();
        let ba = dot(b, a).to_f64();
        prop_assert!(
            approx_eq_f64(ab, ba, 1e-5),
            "a.b={}, b.a={}",
            ab, ba
        );
    }

    /// VXV is anti-commutative: a×b = -(b×a)
    #[test]
    fn vxv_anti_commutative(a in small_vector(), b in small_vector()) {
        let axb = vxv(a, b);
        let bxa = vxv(b, a);
        prop_assert!(
            approx_eq_f64(axb.x.to_f64(), -bxa.x.to_f64(), 1e-4)
            && approx_eq_f64(axb.y.to_f64(), -bxa.y.to_f64(), 1e-4)
            && approx_eq_f64(axb.z.to_f64(), -bxa.z.to_f64(), 1e-4),
            "a×b={:?}, -(b×a)=({}, {}, {})",
            axb, -bxa.x.to_f64(), -bxa.y.to_f64(), -bxa.z.to_f64()
        );
    }

    /// Cross product is orthogonal to both inputs: (a×b).a = 0 and (a×b).b = 0
    #[test]
    fn vxv_orthogonal(a in small_vector(), b in small_vector()) {
        let cross = vxv(a, b);
        let dot_a = dot(cross, a).to_f64();
        let dot_b = dot(cross, b).to_f64();
        prop_assert!(
            dot_a.abs() < 1e-2,
            "(a×b).a = {} (expected 0)", dot_a
        );
        prop_assert!(
            dot_b.abs() < 1e-2,
            "(a×b).b = {} (expected 0)", dot_b
        );
    }

    /// Self cross product is zero: a×a = 0
    #[test]
    fn vxv_self_zero(a in small_vector()) {
        let cross = vxv(a, a);
        prop_assert!(
            cross.x.to_f64().abs() < 1e-5
            && cross.y.to_f64().abs() < 1e-5
            && cross.z.to_f64().abs() < 1e-5,
            "a×a = ({}, {}, {})",
            cross.x.to_f64(), cross.y.to_f64(), cross.z.to_f64()
        );
    }

    /// VAD then VSU is identity: (a+b)-b ≈ a
    #[test]
    fn vad_vsu_roundtrip(a in small_vector(), b in small_vector()) {
        let sum = vad(a, b);
        let back = vsu(sum, b);
        prop_assert!(
            approx_eq_f64(back.x.to_f64(), a.x.to_f64(), 1e-4)
            && approx_eq_f64(back.y.to_f64(), a.y.to_f64(), 1e-4)
            && approx_eq_f64(back.z.to_f64(), a.z.to_f64(), 1e-4),
            "(a+b)-b = ({}, {}, {}), expected ({}, {}, {})",
            back.x.to_f64(), back.y.to_f64(), back.z.to_f64(),
            a.x.to_f64(), a.y.to_f64(), a.z.to_f64()
        );
    }

    /// ABVAL is non-negative.
    #[test]
    fn abval_non_negative(v in small_vector()) {
        let mag = abval(v).to_f64();
        prop_assert!(
            mag >= -1e-8,
            "|v| = {} (expected >= 0)", mag
        );
    }
}

#[test]
fn test_dot_known_345() {
    // {0.3, 0.4, 0} . {0.3, 0.4, 0} = 0.09 + 0.16 = 0.25
    let v = AgcVector::from_f64(0.3, 0.4, 0.0);
    let d = dot(v, v);
    assert!(
        (d.to_f64() - 0.25).abs() < 1e-3,
        "v.v = {}",
        d.to_f64()
    );
}

#[test]
fn test_abval_known_345() {
    let v = AgcVector::from_f64(0.3, 0.4, 0.0);
    let mag = abval(v);
    assert!(
        (mag.to_f64() - 0.5).abs() < 1e-2,
        "|v| = {}",
        mag.to_f64()
    );
}

#[test]
fn test_vxv_basis_vectors() {
    // i × j = k (scaled by 0.5 due to AGC fraction multiply)
    let i = AgcVector::from_f64(0.5, 0.0, 0.0);
    let j = AgcVector::from_f64(0.0, 0.5, 0.0);
    let cross = vxv(i, j);
    // 0.5 * 0.5 = 0.25 in the Z component
    assert!(
        (cross.z.to_f64() - 0.25).abs() < 1e-4,
        "i×j z = {}",
        cross.z.to_f64()
    );
    assert!(cross.x.to_f64().abs() < 1e-6, "i×j x = {}", cross.x.to_f64());
    assert!(cross.y.to_f64().abs() < 1e-6, "i×j y = {}", cross.y.to_f64());
}
