//! Vector and matrix operations mirroring the AGC interpreter.
//!
//! The AGC represents vectors as three DP components stored in
//! MPAC (X), MPAC+3 (Y), MPAC+5 (Z) — six consecutive words total.
//! Matrices are stored as three row vectors (9 DP values, 18 words).
//!
//! Reference: Comanche055/INTERPRETER.agc
//!   DOTSUB (line 967), VXV (line 1265), UNIT (line 2203), ABVAL (line 2353)

use crate::arithmetic::{dad, dmp};
use crate::ones_complement::DpWord;
use crate::sqrt::agc_sqrt;

/// A 3-component vector in AGC double-precision.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AgcVector {
    pub x: DpWord,
    pub y: DpWord,
    pub z: DpWord,
}

/// A 3x3 matrix in AGC double-precision (stored row-major).
#[derive(Clone, Copy, Debug)]
pub struct AgcMatrix {
    pub rows: [AgcVector; 3],
}

impl AgcVector {
    pub const ZERO: AgcVector = AgcVector {
        x: DpWord::ZERO,
        y: DpWord::ZERO,
        z: DpWord::ZERO,
    };

    pub fn new(x: DpWord, y: DpWord, z: DpWord) -> Self {
        AgcVector { x, y, z }
    }

    pub fn from_f64(x: f64, y: f64, z: f64) -> Self {
        AgcVector {
            x: DpWord::from_f64(x),
            y: DpWord::from_f64(y),
            z: DpWord::from_f64(z),
        }
    }
}

/// VAD: Vector Add (INTERPRETER.agc line 780).
///
/// Component-wise addition of two vectors.
pub fn vad(a: AgcVector, b: AgcVector) -> AgcVector {
    let (x, _) = dad(a.x, b.x);
    let (y, _) = dad(a.y, b.y);
    let (z, _) = dad(a.z, b.z);
    AgcVector::new(x, y, z)
}

/// VSU: Vector Subtract.
///
/// Component-wise subtraction: a - b.
pub fn vsu(a: AgcVector, b: AgcVector) -> AgcVector {
    let (x, _) = crate::arithmetic::dsu(a.x, b.x);
    let (y, _) = crate::arithmetic::dsu(a.y, b.y);
    let (z, _) = crate::arithmetic::dsu(a.z, b.z);
    AgcVector::new(x, y, z)
}

/// VXSC: Vector times Scalar (INTERPRETER.agc line 1198).
///
/// Multiply each component of a vector by a scalar.
pub fn vxsc(v: AgcVector, s: DpWord) -> AgcVector {
    AgcVector::new(dmp(v.x, s), dmp(v.y, s), dmp(v.z, s))
}

/// DOT: Dot product (INTERPRETER.agc DOTSUB, line 967).
///
/// Computes a.x*b.x + a.y*b.y + a.z*b.z using DMPSUB for each
/// component pair and accumulating via DAS.
pub fn dot(a: AgcVector, b: AgcVector) -> DpWord {
    let px = dmp(a.x, b.x);
    let py = dmp(a.y, b.y);
    let pz = dmp(a.z, b.z);

    let (sum_xy, _) = dad(px, py);
    let (result, _) = dad(sum_xy, pz);
    result
}

/// VXV: Vector Cross Product (INTERPRETER.agc line 1265).
///
/// Computes a × b = (a.y*b.z - a.z*b.y, a.z*b.x - a.x*b.z, a.x*b.y - a.y*b.x)
pub fn vxv(a: AgcVector, b: AgcVector) -> AgcVector {
    // X = a.y*b.z - a.z*b.y
    let x1 = dmp(a.y, b.z);
    let x2 = dmp(a.z, b.y);
    let (x, _) = crate::arithmetic::dsu(x1, x2);

    // Y = a.z*b.x - a.x*b.z
    let y1 = dmp(a.z, b.x);
    let y2 = dmp(a.x, b.z);
    let (y, _) = crate::arithmetic::dsu(y1, y2);

    // Z = a.x*b.y - a.y*b.x
    let z1 = dmp(a.x, b.y);
    let z2 = dmp(a.y, b.x);
    let (z, _) = crate::arithmetic::dsu(z1, z2);

    AgcVector::new(x, y, z)
}

/// ABVAL: Vector magnitude (INTERPRETER.agc line 2353).
///
/// Computes |v| = sqrt(v.x^2 + v.y^2 + v.z^2).
/// Uses VSQ (VSQSUB, line 2473) internally: dot product with self.
pub fn abval(v: AgcVector) -> DpWord {
    let mag_sq = dot(v, v);
    agc_sqrt(mag_sq)
}

/// UNIT: Unit vector (INTERPRETER.agc line 2203).
///
/// The AGC's UNIT returns a half-unit vector (v / (2*|v|)) when |v| >= 0.5,
/// because the full unit vector could have components of 1.0 which overflows
/// the (-1, 1) fractional range. When |v| < 0.5, it returns v / |v|.
///
/// Returns (result, is_half) where is_half indicates half-unit scaling.
/// Returns the zero vector if |v| ≈ 0.
pub fn unit(v: AgcVector) -> AgcVector {
    let mag = abval(v);
    let mag_f = mag.to_f64();

    if mag_f < 1e-10 {
        return AgcVector::ZERO;
    }

    // The AGC divides by 2*|v| when |v| >= 0.5 to avoid overflow.
    // Even slightly below 0.5, the resulting unit vector can have components
    // near 1.0 (e.g., |v|=0.5, component=0.5 → ratio=1.0). Using a threshold
    // of 0.25 matches the AGC's UNIT behavior which normalizes then checks.
    let half_unit = mag_f >= 0.25;
    let divisor_f = if half_unit {
        (2.0 * mag_f).min(0.99999)
    } else {
        mag_f
    };
    let divisor = DpWord::from_f64(divisor_f);

    AgcVector::new(
        crate::arithmetic::ddv(v.x, divisor),
        crate::arithmetic::ddv(v.y, divisor),
        crate::arithmetic::ddv(v.z, divisor),
    )
}

/// MXV: Matrix times Vector (INTERPRETER.agc line 1146).
///
/// Multiplies a 3x3 matrix by a column vector.
/// result[i] = dot(matrix.row[i], vector)
pub fn mxv(m: AgcMatrix, v: AgcVector) -> AgcVector {
    AgcVector::new(
        dot(m.rows[0], v),
        dot(m.rows[1], v),
        dot(m.rows[2], v),
    )
}

/// VXM: Vector times Matrix (INTERPRETER.agc line 1150).
///
/// Multiplies a row vector by a 3x3 matrix (transpose of MXV).
/// result[j] = dot(vector, matrix.column[j])
///
/// The AGC implements this by using DOTSUB with DOTINC=6
/// to dot MPAC with successive matrix columns.
pub fn vxm(v: AgcVector, m: AgcMatrix) -> AgcVector {
    // Extract columns
    let col0 = AgcVector::new(m.rows[0].x, m.rows[1].x, m.rows[2].x);
    let col1 = AgcVector::new(m.rows[0].y, m.rows[1].y, m.rows[2].y);
    let col2 = AgcVector::new(m.rows[0].z, m.rows[1].z, m.rows[2].z);

    AgcVector::new(dot(v, col0), dot(v, col1), dot(v, col2))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq_dp(a: DpWord, b: DpWord, tol: f64) -> bool {
        (a.to_f64() - b.to_f64()).abs() < tol
    }

    fn approx_eq_vec(a: AgcVector, b: AgcVector, tol: f64) -> bool {
        approx_eq_dp(a.x, b.x, tol)
            && approx_eq_dp(a.y, b.y, tol)
            && approx_eq_dp(a.z, b.z, tol)
    }

    #[test]
    fn test_dot_orthogonal() {
        // i . j = 0
        let i = AgcVector::from_f64(0.5, 0.0, 0.0);
        let j = AgcVector::from_f64(0.0, 0.5, 0.0);
        let result = dot(i, j);
        assert!(result.to_f64().abs() < 1e-7, "i.j = {}", result.to_f64());
    }

    #[test]
    fn test_dot_parallel() {
        let v = AgcVector::from_f64(0.3, 0.4, 0.0);
        let d = dot(v, v);
        // 0.3^2 + 0.4^2 = 0.09 + 0.16 = 0.25
        assert!(
            (d.to_f64() - 0.25).abs() < 1e-4,
            "v.v = {} (expected 0.25)",
            d.to_f64()
        );
    }

    #[test]
    fn test_dot_commutative() {
        let a = AgcVector::from_f64(0.3, -0.2, 0.1);
        let b = AgcVector::from_f64(-0.1, 0.4, 0.3);
        let ab = dot(a, b);
        let ba = dot(b, a);
        assert!(
            (ab.to_f64() - ba.to_f64()).abs() < 1e-6,
            "a.b={} != b.a={}",
            ab.to_f64(),
            ba.to_f64()
        );
    }

    #[test]
    fn test_vxv_anti_commutative() {
        let a = AgcVector::from_f64(0.3, -0.2, 0.1);
        let b = AgcVector::from_f64(-0.1, 0.4, 0.3);
        let axb = vxv(a, b);
        let bxa = vxv(b, a);
        // a×b should equal -(b×a)
        assert!(
            approx_eq_vec(
                axb,
                AgcVector::new(bxa.x.negate(), bxa.y.negate(), bxa.z.negate()),
                1e-4
            ),
            "a×b = {:?}, b×a = {:?}",
            axb,
            bxa
        );
    }

    #[test]
    fn test_vxv_orthogonal_to_inputs() {
        let a = AgcVector::from_f64(0.3, -0.2, 0.1);
        let b = AgcVector::from_f64(-0.1, 0.4, 0.3);
        let cross = vxv(a, b);

        // (a×b) . a = 0
        let dot_a = dot(cross, a);
        assert!(
            dot_a.to_f64().abs() < 1e-3,
            "(a×b).a = {}",
            dot_a.to_f64()
        );

        // (a×b) . b = 0
        let dot_b = dot(cross, b);
        assert!(
            dot_b.to_f64().abs() < 1e-3,
            "(a×b).b = {}",
            dot_b.to_f64()
        );
    }

    #[test]
    fn test_vxv_self_is_zero() {
        let a = AgcVector::from_f64(0.3, -0.2, 0.1);
        let cross = vxv(a, a);
        assert!(
            approx_eq_vec(cross, AgcVector::ZERO, 1e-5),
            "a×a = {:?} (expected zero)",
            cross
        );
    }

    #[test]
    fn test_abval_345() {
        // |{0.3, 0.4, 0}| = 0.5
        let v = AgcVector::from_f64(0.3, 0.4, 0.0);
        let mag = abval(v);
        assert!(
            (mag.to_f64() - 0.5).abs() < 1e-3,
            "|v| = {} (expected 0.5)",
            mag.to_f64()
        );
    }

    #[test]
    fn test_unit_direction_preserved() {
        // UNIT returns a half-unit vector: v / (2*|v|).
        // For v = (0.3, 0.4, 0), |v| = 0.5.
        // half-unit = v / (2*0.5) = v / 1.0 ≈ (0.3, 0.4, 0)
        let v = AgcVector::from_f64(0.3, 0.4, 0.0);
        let u = unit(v);

        // For |v| >= 0.25 (half-unit mode), result = v / (2*|v|)
        // = (0.3, 0.4, 0) / 1.0 ≈ (0.3, 0.4, 0)
        // The exact values depend on rounding, but direction is preserved.

        // Verify direction: uy/ux = vy/vx = 4/3
        assert!(u.x.to_f64().abs() > 0.01, "unit x too small: {}", u.x.to_f64());
        let ratio = u.y.to_f64() / u.x.to_f64();
        assert!(
            (ratio - 4.0 / 3.0).abs() < 0.1,
            "direction ratio = {} (expected 4/3)",
            ratio
        );

        // Verify Z component is near zero
        assert!(
            u.z.to_f64().abs() < 1e-5,
            "z component should be 0, got {}",
            u.z.to_f64()
        );
    }

    #[test]
    fn test_vad_vsu_roundtrip() {
        let a = AgcVector::from_f64(0.3, -0.2, 0.1);
        let b = AgcVector::from_f64(-0.1, 0.4, 0.3);
        let sum = vad(a, b);
        let back = vsu(sum, b);
        assert!(
            approx_eq_vec(back, a, 1e-5),
            "(a+b)-b = {:?}, expected {:?}",
            back,
            a
        );
    }

    #[test]
    fn test_mxv_identity() {
        let identity = AgcMatrix {
            rows: [
                AgcVector::from_f64(0.5, 0.0, 0.0),
                AgcVector::from_f64(0.0, 0.5, 0.0),
                AgcVector::from_f64(0.0, 0.0, 0.5),
            ],
        };
        // Identity matrix (scaled by 0.5) times vector gives vector/2
        // because MXV does dot products, and the "1.0" in the identity
        // is represented as 0.5 in AGC fractional scaling
        let v = AgcVector::from_f64(0.6, -0.4, 0.2);
        let result = mxv(identity, v);
        // Each result component = 0.5 * v_component (fractional multiply)
        assert!(
            (result.x.to_f64() - 0.3).abs() < 1e-4,
            "identity*v x: {}",
            result.x.to_f64()
        );
    }
}
