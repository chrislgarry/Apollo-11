//! Polynomial computation engine mirroring the AGC's POLY pseudo-instruction.
//!
//! The POLY instruction computes a polynomial using Horner's method:
//!   P(x) = C[n]*x^n + C[n-1]*x^(n-1) + ... + C[1]*x + C[0]
//!
//! In the AGC source, the degree and coefficients follow the `TC POLY` call:
//!   TC   POLY
//!   DEC  N        (degree)
//!   2DEC C[0]     (constant term)
//!   2DEC C[1]     (x coefficient)
//!   ...
//!   2DEC C[N]     (x^N coefficient)
//!
//! The computation uses Horner form:
//!   result = C[0] + x*(C[1] + x*(C[2] + ... x*C[N]))
//!
//! Reference: Comanche055/INTERPRETER.agc, POLY routine

use crate::arithmetic::dmp;
use crate::ones_complement::DpWord;

/// Compute a polynomial using Horner's method with AGC arithmetic.
///
/// `coeffs` is ordered [C0, C1, C2, ..., CN] where C0 is the constant term.
/// `x` is the argument (DP fractional value).
///
/// Horner computation: start from CN, multiply by x, add C(N-1), repeat.
pub fn poly_horner(x: DpWord, coeffs: &[DpWord]) -> DpWord {
    assert!(!coeffs.is_empty(), "polynomial must have at least one coefficient");

    if coeffs.len() == 1 {
        return coeffs[0];
    }

    // Start with the highest-degree coefficient
    let mut result = coeffs[coeffs.len() - 1];

    // Work backwards through coefficients (Horner's method)
    for i in (0..coeffs.len() - 1).rev() {
        result = dmp(result, x);
        // Add coefficient
        let (sum, _ovf) = crate::arithmetic::dad(result, coeffs[i]);
        result = sum;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_polynomial() {
        let x = DpWord::from_f64(0.5);
        let coeffs = [DpWord::from_f64(0.25)];
        let result = poly_horner(x, &coeffs);
        assert!((result.to_f64() - 0.25).abs() < 1e-7);
    }

    #[test]
    fn test_linear_polynomial() {
        // P(x) = 0.1 + 0.5*x
        // P(0.4) = 0.1 + 0.2 = 0.3
        let x = DpWord::from_f64(0.4);
        let coeffs = [DpWord::from_f64(0.1), DpWord::from_f64(0.5)];
        let result = poly_horner(x, &coeffs);
        assert!((result.to_f64() - 0.3).abs() < 1e-4);
    }

    #[test]
    fn test_quadratic_polynomial() {
        // P(x) = 0.5*x^2, so P(0.5) = 0.125
        let x = DpWord::from_f64(0.5);
        let coeffs = [
            DpWord::ZERO,
            DpWord::ZERO,
            DpWord::from_f64(0.5),
        ];
        let result = poly_horner(x, &coeffs);
        assert!((result.to_f64() - 0.125).abs() < 1e-4);
    }
}
