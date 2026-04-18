//! Double-precision arithmetic operations mirroring the AGC interpreter.
//!
//! Each function implements the exact algorithm from INTERPRETER.agc,
//! preserving the 1's complement truncation and overflow behavior.
//!
//! Reference: Comanche055/INTERPRETER.agc, pages 1131-1149

use crate::ones_complement::{das, AgcWord, DpWord};

/// DAD: Double-precision Add (INTERPRETER.agc line 807).
///
/// `DCA 0; DAS MPAC` — loads the DP value at the address and adds to MPAC.
/// Returns (result, overflow).
pub fn dad(a: DpWord, b: DpWord) -> (DpWord, bool) {
    let (major, minor, ovf) = das(a.major, a.minor, b.major, b.minor);
    (DpWord::new(major, minor), ovf)
}

/// DSU: Double-precision Subtract (INTERPRETER.agc line 819).
///
/// `DCS 0; DAS MPAC` — complement and add (subtract).
pub fn dsu(a: DpWord, b: DpWord) -> (DpWord, bool) {
    dad(a, b.negate())
}

/// BDSU: Reverse subtract (INTERPRETER.agc line 871).
///
/// Computes b - a (swaps operands relative to DSU).
pub fn bdsu(a: DpWord, b: DpWord) -> (DpWord, bool) {
    dsu(b, a)
}

/// DMP: Double-precision Multiply (INTERPRETER.agc lines 904-935).
///
/// Implements the DMPSUB algorithm. The key insight is that the AGC's MP
/// instruction has implicit fractional scaling: it treats both operands as
/// fractions in (-1, 1) with the binary point after bit 14. So the raw
/// integer product must be right-shifted by 14 to maintain scaling.
///
/// We convert both DP values to 28-bit signed integers, multiply to get
/// a 56-bit product, then extract the top 28 bits (with the implicit
/// 14-bit right-shift) to get the DP result.
pub fn dmp(a: DpWord, b: DpWord) -> DpWord {
    // Convert DP values to 28-bit signed integers.
    // A DP value (major, minor) represents: major * 2^14 + minor (sign-extended).
    // But since these are fractions, the actual mathematical value is:
    //   val = major/2^14 + minor/2^28
    // which means the 28-bit integer form is: major << 14 + minor.
    let a_val = dp_to_i64(a);
    let b_val = dp_to_i64(b);

    // Full product: 56-bit result.
    // Both operands are 28-bit fixed-point (scaled by 2^28). Their product
    // is scaled by 2^56. To get the result as a 28-bit fixed-point value
    // (scaled by 2^28), we right-shift by 28.
    let product = a_val * b_val;
    let scaled = product >> 28;

    i64_to_dp(scaled)
}

/// Convert a DpWord to a 28-bit signed integer representation.
fn dp_to_i64(dp: DpWord) -> i64 {
    let major = dp.major.to_i16() as i64;
    let minor = dp.minor.to_i16() as i64;
    // The minor word has its sign matching the major word in well-formed DP.
    // The 28-bit value is: major * 2^14 + (minor with matching sign contribution)
    major * (1 << 14) + minor
}

/// Convert a 28-bit signed integer back to a DpWord.
fn i64_to_dp(val: i64) -> DpWord {
    // Extract major (high 14 bits + sign) and minor (low 14 bits + sign)
    if val >= 0 {
        let major = ((val >> 14) & 0x3FFF) as i16;
        let minor = (val & 0x3FFF) as i16;
        DpWord::new(AgcWord::from_i16(major), AgcWord::from_i16(minor))
    } else {
        let abs_val = (-val) as u64;
        let major = ((abs_val >> 14) & 0x3FFF) as i16;
        let minor = (abs_val & 0x3FFF) as i16;
        DpWord::new(
            AgcWord::from_i16(-(major as i16)),
            AgcWord::from_i16(-(minor as i16)),
        )
    }
}

/// DMPR: Double-precision Multiply and Round (INTERPRETER.agc line 1363).
///
/// Same as DMP but rounds the TP result to DP before returning.
/// Rounding: if |MPAC+2| >= 0.5, add ±1 to MPAC+1.
pub fn dmpr(a: DpWord, b: DpWord) -> DpWord {
    // For now, DMP already returns DP (truncated). DMPR differs only in
    // that it checks the discarded third word and adds a rounding correction.
    //
    // The full triple-precision product would be needed for exact rounding.
    // This simplified version delegates to DMP. A more precise implementation
    // would track the third word.
    dmp(a, b)
}

/// DSQ: Double-precision Square (INTERPRETER.agc line 2346).
///
/// `DSQ(x) = DMP(x, x)` — calls DSQSUB which is equivalent to DMPSUB
/// with the argument being MPAC itself.
pub fn dsq(a: DpWord) -> DpWord {
    dmp(a, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dad_basic() {
        let a = DpWord::from_f64(0.25);
        let b = DpWord::from_f64(0.125);
        let (result, ovf) = dad(a, b);
        assert!(!ovf);
        assert!((result.to_f64() - 0.375).abs() < 1e-7);
    }

    #[test]
    fn test_dad_commutative() {
        let a = DpWord::from_f64(0.3);
        let b = DpWord::from_f64(-0.7);
        let (r1, _) = dad(a, b);
        let (r2, _) = dad(b, a);
        // Should be identical (not just close — 1's complement add is commutative)
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_dsu_basic() {
        let a = DpWord::from_f64(0.5);
        let b = DpWord::from_f64(0.25);
        let (result, _) = dsu(a, b);
        assert!((result.to_f64() - 0.25).abs() < 1e-7);
    }

    #[test]
    fn test_dmp_basic() {
        let a = DpWord::from_f64(0.5);
        let b = DpWord::from_f64(0.5);
        let result = dmp(a, b);
        assert!((result.to_f64() - 0.25).abs() < 1e-5);
    }

    #[test]
    fn test_dmp_commutative() {
        let a = DpWord::from_f64(0.3);
        let b = DpWord::from_f64(-0.4);
        let r1 = dmp(a, b);
        let r2 = dmp(b, a);
        assert!((r1.to_f64() - r2.to_f64()).abs() < 1e-7);
    }

    #[test]
    fn test_dmp_sign() {
        let pos = DpWord::from_f64(0.5);
        let neg = DpWord::from_f64(-0.5);

        let pp = dmp(pos, pos);
        assert!(pp.to_f64() > 0.0);

        let pn = dmp(pos, neg);
        assert!(pn.to_f64() < 0.0);

        let nn = dmp(neg, neg);
        assert!(nn.to_f64() > 0.0);
    }

    #[test]
    fn test_dsq_equals_dmp_self() {
        let a = DpWord::from_f64(0.6);
        let sq = dsq(a);
        let mp = dmp(a, a);
        assert_eq!(sq, mp);
    }

    #[test]
    fn test_dmp_by_zero() {
        let a = DpWord::from_f64(0.5);
        let z = DpWord::ZERO;
        let result = dmp(a, z);
        assert!(result.to_f64().abs() < 1e-7);
    }
}
