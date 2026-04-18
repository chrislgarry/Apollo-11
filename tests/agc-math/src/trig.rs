//! Trigonometric functions using the exact AGC Hastings polynomial approximations.
//!
//! The AGC computes sine/cosine using a 4th-order polynomial fitted by
//! Cecil Hastings. The argument is in revolutions (not radians), and the
//! result is half-scaled: SIN(X) computes (1/2)sin(2*pi*X).
//!
//! COSINE uses the identity: COS(X) = SIN(PI/2 - |X|), which in the AGC's
//! revolution scaling is: COS(X) = SIN(QUARTER - |X|) where QUARTER = 0.25.
//!
//! Reference: Comanche055/INTERPRETER.agc lines 2641-2703 (SINE/COSINE)
//!            Comanche055/INTERPRETER.agc lines 2708-2797 (ARCSIN/ARCCOS)

use crate::ones_complement::DpWord;

/// Sine polynomial coefficients from INTERPRETER.agc lines 2684-2688.
///
/// These are the Hastings approximation coefficients for:
///   (1/2)sin(2*pi*x) ≈ x * P(x^2)
/// where P is a 4th-order polynomial.
///
/// The AGC stores these as 2DEC (double-precision decimal) values.
const SINE_COEFFS: [f64; 5] = [
    0.3926990796,   // C0: ≈ pi/8
    -0.6459637111,  // C1
    0.318758717,    // C2
    -0.074780249,   // C3
    0.009694988,    // C4
];

/// Arccos polynomial coefficients from INTERPRETER.agc lines 2760-2767.
///
/// ARCCOS(x) = sqrt(1-x) * P(x) where P is a 7th-order polynomial.
/// The coefficients have scaling factors (B+N) indicating left shifts.
///
/// Original 2DEC values with B+N scaling:
///   +.353553385          (B+0, no shift)
///   -.0483017006  B+1    (shift left 1)
///   +.0200273085  B+2    (shift left 2)
///   -.0112931863  B+3    (shift left 3)
///   +.00695311612 B+4    (shift left 4)
///   -.00384617957 B+5    (shift left 5)
///   +.001501297736 B+6   (shift left 6)
///   -.000284160334 B+7   (shift left 7)
const ARCCOS_COEFFS: [f64; 8] = [
    0.353553385,
    -0.0483017006 * 2.0,
    0.0200273085 * 4.0,
    -0.0112931863 * 8.0,
    0.00695311612 * 16.0,
    -0.00384617957 * 32.0,
    0.001501297736 * 64.0,
    -0.000284160334 * 128.0,
];

/// Compute (1/2)sin(2*pi*x) using the AGC's Hastings polynomial.
///
/// Input: x as a DP fractional value in AGC scaling (revolutions).
/// Output: (1/2)sin(2*pi*x) as a DP fractional value.
///
/// The AGC algorithm (lines 2653-2703):
/// 1. Double the argument
/// 2. If overflow, complement (using sin(x ± pi) = -sin(x))
/// 3. If |arg| > 0.5, reduce using sin(pi - x) = sin(x)
/// 4. Square the argument
/// 5. Use POLY to compute P(x^2) with the Hastings coefficients
/// 6. Multiply by the original argument and shift left 2
pub fn agc_sine(x: f64) -> f64 {
    // Step 1: Double argument (DDOUBL)
    let mut arg = 2.0 * x;

    // Step 2: Handle overflow (wrapping past ±1.0 means we crossed ±pi)
    if arg > 1.0 {
        arg = -(arg - 2.0);
    } else if arg < -1.0 {
        arg = -(arg + 2.0);
    }

    // Step 3: If |arg| > 0.5, reduce: sin(pi - x) = sin(x) in revolution scaling
    if arg > 0.5 {
        arg = 1.0 - arg;
    } else if arg < -0.5 {
        arg = -1.0 - arg;
    }

    // Step 4: Square the argument
    let x_sq = arg * arg;

    // Step 5: Horner polynomial P(x^2) with Hastings coefficients
    let mut poly = SINE_COEFFS[4];
    for i in (0..4).rev() {
        poly = poly * x_sq + SINE_COEFFS[i];
    }

    // Step 6: Multiply by argument and shift left 2 (×4)
    poly * arg * 4.0
}

/// Compute (1/2)cos(2*pi*x) using the identity COS(X) = SIN(QUARTER - |X|).
///
/// In the AGC's revolution scaling, pi/2 = 0.25 revolutions.
pub fn agc_cosine(x: f64) -> f64 {
    // COSINE (line 2641): COS(X) = SIN(PI/2 - |X|)
    // In AGC scaling: QUARTER - |X| where QUARTER = 0.25
    let abs_x = if x < 0.0 {
        // The AGC complements MPAC for negative input (lines 2646-2648)
        -x
    } else {
        x
    };
    // PRESINE (line 2650): add QUARTER
    agc_sine(0.25 - abs_x)
}

/// Compute arccos(x) using the AGC's Hastings polynomial approximation.
///
/// Input: x is the mathematical argument in [-1.0, 1.0].
/// Output: arccos(x) in revolutions (0 to 0.5, where 0.5 = pi radians).
///
/// The AGC internally uses half-scaled input: x_internal = x/2. The formula is:
///   arccos_agc = sqrt(0.5 - x_internal) * P(x_internal)
/// which equals arccos(2 * x_internal) / (2*pi) = arccos(x) / (2*pi).
///
/// The coefficient comment (line 2760) confirms this scaling:
///   "COEFFICIENTS ARE C 2(+I)/PISQRT(2)"
///
/// Reference: Comanche055/INTERPRETER.agc lines 2711-2797
pub fn agc_arccos(x_math: f64) -> f64 {
    let (arg, negate) = if x_math < 0.0 {
        (-x_math, true)
    } else {
        (x_math, false)
    };

    // The AGC internally half-scales the input (ACOSST2: HALF - MPAC)
    let x_internal = arg / 2.0;

    // sqrt(0.5 - x_internal) = sqrt((1 - arg) / 2)
    let sqrt_term = (0.5 - x_internal).max(0.0).sqrt();

    // Horner computation of P(x_internal)
    let mut poly = ARCCOS_COEFFS[7];
    for i in (0..7).rev() {
        poly = poly * x_internal + ARCCOS_COEFFS[i];
    }

    let result = sqrt_term * poly;

    if negate {
        // SUBTR (line 2773): ARCCOS(-x) = pi - ARCCOS(x) = 0.5 - result
        0.5 - result
    } else {
        result
    }
}

/// Compute arcsin(x) using the identity arcsin(x) = pi/2 - arccos(x).
///
/// Input: x is the mathematical argument in [-1.0, 1.0].
/// Output: arcsin(x) in revolutions (where 0.25 = pi/2 radians).
///
/// Reference: ASINEX at line 2779: `DCS MPAC; AD QUARTER; DXCH MPAC`
pub fn agc_arcsin(x_math: f64) -> f64 {
    0.25 - agc_arccos(x_math)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    /// Convert AGC sine output to standard sine value.
    /// AGC returns (1/2)sin(2*pi*x), so multiply by 2 to get sin(2*pi*x).
    fn to_std_sine(agc_result: f64) -> f64 {
        agc_result * 2.0
    }

    #[test]
    fn test_sine_zero() {
        let result = agc_sine(0.0);
        assert!(result.abs() < 1e-10, "sin(0) should be 0, got {}", result);
    }

    #[test]
    fn test_sine_quarter() {
        // sin(2*pi*0.25) = sin(pi/2) = 1.0
        // AGC returns half-scaled: 0.5
        let result = agc_sine(0.25);
        assert!(
            (result - 0.5).abs() < 1e-6,
            "sin(pi/2)/2 should be 0.5, got {}",
            result
        );
    }

    #[test]
    fn test_sine_eighth() {
        // sin(2*pi*0.125) = sin(pi/4) = sqrt(2)/2 ≈ 0.7071
        // Half-scaled: ≈ 0.3536
        let result = agc_sine(0.125);
        let expected = (PI / 4.0).sin() / 2.0;
        assert!(
            (result - expected).abs() < 1e-5,
            "sin(pi/4)/2: expected {}, got {}",
            expected,
            result
        );
    }

    #[test]
    fn test_cosine_zero() {
        // cos(0) = 1.0, half-scaled = 0.5
        let result = agc_cosine(0.0);
        assert!(
            (result - 0.5).abs() < 1e-6,
            "cos(0)/2 should be 0.5, got {}",
            result
        );
    }

    #[test]
    fn test_cosine_quarter() {
        // cos(2*pi*0.25) = cos(pi/2) = 0
        let result = agc_cosine(0.25);
        assert!(result.abs() < 1e-6, "cos(pi/2) should be 0, got {}", result);
    }

    #[test]
    fn test_pythagorean_identity() {
        // sin^2(x) + cos^2(x) = 1 for various x
        // In AGC scaling: (2*sin_result)^2 + (2*cos_result)^2 = 1
        for i in 0..16 {
            let x = i as f64 / 64.0; // 0 to 0.25 in steps
            let s = to_std_sine(agc_sine(x));
            let c = to_std_sine(agc_cosine(x)); // cosine also half-scaled, so *2
            let sum = s * s + c * c;
            assert!(
                (sum - 1.0).abs() < 1e-4,
                "sin^2 + cos^2 at x={}: got {}",
                x,
                sum
            );
        }
    }

    #[test]
    fn test_sine_256_point_sweep() {
        // Sweep 256 points across the full domain and check against f64 sin
        for i in 0..256 {
            let x = (i as f64 / 256.0) - 0.5; // range [-0.5, 0.5)
            let agc = to_std_sine(agc_sine(x));
            let reference = (2.0 * PI * x).sin();
            assert!(
                (agc - reference).abs() < 1e-4,
                "sine mismatch at x={}: agc={}, ref={}",
                x,
                agc,
                reference
            );
        }
    }

    #[test]
    fn test_arccos_zero() {
        // arccos(0) = pi/2, in revolution scaling = 0.25
        let result = agc_arccos(0.0);
        assert!(
            (result - 0.25).abs() < 1e-5,
            "arccos(0) should be 0.25 rev, got {}",
            result
        );
    }

    #[test]
    fn test_arccos_one() {
        // arccos(1) = 0
        // But AGC can't represent exactly 1.0 (POSMAX ≈ 0.99994)
        let result = agc_arccos(0.99);
        assert!(result.abs() < 0.05, "arccos(~1) should be ~0, got {}", result);
    }

    #[test]
    fn test_arcsin_arccos_complementary() {
        // arcsin(x) + arccos(x) = pi/2 = 0.25 revolutions
        for i in 1..10 {
            let x = i as f64 / 10.0;
            let sum = agc_arcsin(x) + agc_arccos(x);
            assert!(
                (sum - 0.25).abs() < 1e-4,
                "arcsin + arccos at x={}: got {} (expected 0.25)",
                x,
                sum
            );
        }
    }

    #[test]
    fn test_sine_arcsin_roundtrip() {
        // sin(arcsin(x)) ≈ x
        // agc_arcsin(x) returns revolutions, agc_sine takes revolutions
        // agc_sine returns half-scaled, so multiply by 2 to get mathematical sine
        for i in 1..10 {
            let x = i as f64 / 10.0; // 0.1 to 0.9
            let asin_result = agc_arcsin(x); // arcsin(x) in revolutions
            let roundtrip = to_std_sine(agc_sine(asin_result)); // sin(arcsin(x)) = x
            assert!(
                (roundtrip - x).abs() < 1e-3,
                "sin(arcsin({})) = {}, expected {}",
                x,
                roundtrip,
                x
            );
        }
    }
}
