//! 1's complement arithmetic types for the Apollo Guidance Computer (Block II).
//!
//! The AGC uses 15-bit 1's complement words with a 16th parity bit inaccessible
//! to software. This module provides `AgcWord` (single precision), `DpWord`
//! (double precision), and `TpWord` (triple precision) types that enforce
//! AGC semantics at the type level.
//!
//! Key differences from 2's complement (modern CPUs):
//! - Two representations of zero: +0 (octal 00000) and -0 (octal 77777)
//! - Range: -16383 to +16383 (not -16384 to +16383)
//! - Overflow: 16-bit accumulator, overflow when bits 14 and 15 disagree
//! - CCS instruction distinguishes +0 from -0

use std::fmt;

/// A 15-bit 1's complement word.
///
/// Internally stored as the raw 15-bit pattern in the low 15 bits of a u16.
/// Bit 14 is the sign bit (0 = positive, 1 = negative).
///
/// Octal 00000 = +0, octal 77777 = -0 (all ones).
/// Positive values: sign=0, magnitude in bits 0-13.
/// Negative values: bitwise complement of the positive value.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgcWord(u16);

/// Double-precision: two AgcWords.
///
/// The major word carries the sign and high 14 magnitude bits.
/// The minor word's sign bit must agree with the major word's sign.
/// Together they provide 28 magnitude bits.
///
/// DP value = major * 2^(-14) + minor * 2^(-28) (in fractional scaling)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DpWord {
    pub major: AgcWord,
    pub minor: AgcWord,
}

/// Triple-precision: three AgcWords, as used internally by the MPAC.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TpWord {
    pub hi: AgcWord,
    pub mid: AgcWord,
    pub lo: AgcWord,
}

/// The 16-bit accumulator (A register). Overflow is possible.
/// Bits 15 and 14 are the two sign bits in the overflow-capable representation.
/// Overflow occurs when they disagree.
#[derive(Clone, Copy, Debug)]
pub struct Accumulator(pub i16);

// ---------------------------------------------------------------------------
// AgcWord
// ---------------------------------------------------------------------------

impl AgcWord {
    /// Positive zero: octal 00000
    pub const POS_ZERO: AgcWord = AgcWord(0);

    /// Negative zero: octal 77777 (all 15 bits set)
    pub const NEG_ZERO: AgcWord = AgcWord(0x7FFF);

    /// Maximum positive value: octal 37777 = 16383
    pub const POS_MAX: AgcWord = AgcWord(0x3FFF);

    /// Maximum negative value (most negative): octal 40000 = -16383
    pub const NEG_MAX: AgcWord = AgcWord(0x4000);

    /// Mask for the 15-bit word
    pub const MASK: u16 = 0x7FFF;

    /// Sign bit position (bit 14)
    const SIGN_BIT: u16 = 0x4000;

    /// Magnitude mask (bits 0-13)
    const MAG_MASK: u16 = 0x3FFF;

    /// Create from a raw 15-bit pattern in const context. No runtime check.
    pub const fn from_raw_const(raw: u16) -> Self {
        // Can't use assert! in const fn on stable, but we trust const callers.
        AgcWord(raw)
    }

    /// Create an AgcWord from a raw 15-bit pattern. Panics if value > 0x7FFF.
    pub fn from_raw(raw: u16) -> Self {
        assert!(raw <= Self::MASK, "raw value {:#06o} exceeds 15 bits", raw);
        AgcWord(raw)
    }

    /// Create an AgcWord from an octal literal.
    pub fn from_octal(oct: u16) -> Self {
        Self::from_raw(oct)
    }

    /// Create an AgcWord from a signed integer.
    ///
    /// Maps 0 to +0 (not -0). For -0, use `AgcWord::NEG_ZERO`.
    /// Panics if value is outside [-16383, 16383].
    pub fn from_i16(val: i16) -> Self {
        assert!(
            (-16383..=16383).contains(&val),
            "value {} out of AGC range [-16383, 16383]",
            val
        );
        if val >= 0 {
            AgcWord(val as u16)
        } else {
            // 1's complement negation: bitwise NOT of magnitude, masked to 15 bits
            AgcWord((!(-val) as u16) & Self::MASK)
        }
    }

    /// The raw 15-bit pattern.
    pub fn raw(self) -> u16 {
        self.0
    }

    /// True if the sign bit (bit 14) is clear — value is positive or +0.
    pub fn is_positive(self) -> bool {
        self.0 & Self::SIGN_BIT == 0
    }

    /// True if the sign bit (bit 14) is set — value is negative or -0.
    pub fn is_negative(self) -> bool {
        self.0 & Self::SIGN_BIT != 0
    }

    /// True if this is either +0 or -0.
    pub fn is_zero(self) -> bool {
        self == Self::POS_ZERO || self == Self::NEG_ZERO
    }

    /// True if this is positive zero specifically.
    pub fn is_pos_zero(self) -> bool {
        self == Self::POS_ZERO
    }

    /// True if this is negative zero specifically.
    pub fn is_neg_zero(self) -> bool {
        self == Self::NEG_ZERO
    }

    /// The magnitude (absolute value) as a u16, range 0-16383.
    pub fn magnitude(self) -> u16 {
        if self.is_positive() {
            self.0
        } else {
            (!self.0) & Self::MASK
        }
    }

    /// Convert to a signed integer. Both +0 and -0 map to 0.
    pub fn to_i16(self) -> i16 {
        if self.is_positive() {
            self.0 as i16
        } else {
            -((!self.0 & Self::MASK) as i16)
        }
    }

    /// 1's complement negation: flip all 15 bits.
    /// +0 becomes -0 and vice versa.
    pub fn negate(self) -> Self {
        AgcWord((!self.0) & Self::MASK)
    }

    /// 1's complement absolute value: if negative, negate; if positive, keep.
    /// -0 becomes +0.
    pub fn abs(self) -> Self {
        if self.is_negative() {
            self.negate()
        } else {
            self
        }
    }

    /// Perform CCS (Count, Compare, and Skip) on this word.
    ///
    /// Returns the branch index (0-3) and the "diminished absolute value":
    /// - A > 0:  branch 0, DABS = A - 1
    /// - A = +0: branch 1, DABS = +0
    /// - A < 0:  branch 2, DABS = |A| - 1
    /// - A = -0: branch 3, DABS = +0
    pub fn ccs(self) -> (u8, AgcWord) {
        if self.is_pos_zero() {
            (1, Self::POS_ZERO)
        } else if self.is_neg_zero() {
            (3, Self::POS_ZERO)
        } else if self.is_positive() {
            // Positive non-zero: diminish by 1
            (0, AgcWord(self.0 - 1))
        } else {
            // Negative non-zero: magnitude - 1
            let mag = self.magnitude();
            (2, AgcWord(mag - 1))
        }
    }

    /// Add two AgcWords, returning an Accumulator (which may overflow).
    pub fn add(self, other: AgcWord) -> Accumulator {
        // Convert to signed, add, then handle end-around carry
        let a = self.to_i32();
        let b = other.to_i32();
        let sum = a + b;
        Accumulator::from_ones_complement_i32(sum)
    }

    /// Internal: convert to i32 for intermediate arithmetic.
    fn to_i32(self) -> i32 {
        self.to_i16() as i32
    }
}

impl fmt::Debug for AgcWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AgcWord({:05o})", self.0)
    }
}

impl fmt::Display for AgcWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_neg_zero() {
            write!(f, "-0")
        } else {
            write!(f, "{}", self.to_i16())
        }
    }
}

// ---------------------------------------------------------------------------
// Accumulator
// ---------------------------------------------------------------------------

impl Accumulator {
    /// Create from a raw 16-bit value (the full A register content).
    pub fn from_raw(raw: i16) -> Self {
        Accumulator(raw)
    }

    /// Create from a 1's complement addition result.
    ///
    /// In 1's complement, carry out of the MSB wraps around as +1 (end-around carry).
    /// The result is a 16-bit value where bits 15 and 14 form the overflow indicator.
    fn from_ones_complement_i32(val: i32) -> Self {
        // The AGC accumulator is 16 bits. In 1's complement addition:
        // - If sum is in [-16383, 16383], no overflow
        // - If sum > 16383 (positive overflow), bit 15=0, bit 14=1 pattern
        // - If sum < -16383 (negative overflow), bit 15=1, bit 14=0 pattern
        //
        // We store the result as a 16-bit signed value where the top two bits
        // indicate overflow state.
        Accumulator(val.clamp(-32767, 32767) as i16)
    }

    /// True if the accumulator is in overflow (bits 14 and 15 disagree).
    pub fn has_overflow(&self) -> bool {
        let bit15 = (self.0 >> 15) & 1;
        let bit14 = (self.0 >> 14) & 1;
        bit15 != bit14
    }

    /// True if positive overflow (value > POSMAX).
    pub fn is_positive_overflow(&self) -> bool {
        self.0 > AgcWord::POS_MAX.raw() as i16
    }

    /// True if negative overflow (value < -POSMAX).
    pub fn is_negative_overflow(&self) -> bool {
        self.has_overflow() && (self.0 >> 15) & 1 == 1
    }

    /// Extract the 15-bit result, discarding overflow.
    /// If overflow occurred, returns the corrected value after
    /// end-around carry (the TS instruction behavior).
    pub fn to_agc_word(&self) -> AgcWord {
        if !self.has_overflow() {
            AgcWord::from_raw((self.0 as u16) & AgcWord::MASK)
        } else if self.is_positive_overflow() {
            // Positive overflow: wrap to +1 (carry)
            AgcWord::from_i16(1)
        } else {
            // Negative overflow: wrap to -1
            AgcWord::from_i16(-1)
        }
    }

    /// The raw 16-bit register value.
    pub fn raw(&self) -> i16 {
        self.0
    }
}

// ---------------------------------------------------------------------------
// DpWord
// ---------------------------------------------------------------------------

impl DpWord {
    /// Double-precision zero.
    pub const ZERO: DpWord = DpWord {
        major: AgcWord::POS_ZERO,
        minor: AgcWord::POS_ZERO,
    };

    /// Create a DpWord from major and minor AgcWords.
    /// The sign bits should agree for a well-formed DP value.
    pub fn new(major: AgcWord, minor: AgcWord) -> Self {
        DpWord { major, minor }
    }

    /// Create a DpWord from a floating-point value in the range (-1.0, 1.0).
    ///
    /// The AGC treats DP values as fractions with the binary point after bit 14
    /// of the major word. So 0.5 = (HALF, +0) and -0.5 = (-HALF, -0).
    pub fn from_f64(val: f64) -> Self {
        assert!(
            val.abs() < 1.0,
            "DP fractional value {} out of range (-1.0, 1.0)",
            val
        );

        if val == 0.0 {
            return Self::ZERO;
        }

        // Scale to 28-bit integer range: val * 2^28
        let scaled = (val * (1i64 << 28) as f64).round() as i32;

        let sign_negative = scaled < 0;
        let abs_scaled = scaled.unsigned_abs();

        // Major = top 14 magnitude bits, minor = low 14 magnitude bits
        let major_mag = ((abs_scaled >> 14) & 0x3FFF) as u16;
        let minor_mag = (abs_scaled & 0x3FFF) as u16;

        if sign_negative {
            DpWord {
                major: AgcWord::from_raw((!major_mag) & 0x7FFF),
                minor: AgcWord::from_raw((!minor_mag) & 0x7FFF),
            }
        } else {
            DpWord {
                major: AgcWord::from_raw(major_mag),
                minor: AgcWord::from_raw(minor_mag),
            }
        }
    }

    /// Convert to f64 fractional value.
    pub fn to_f64(self) -> f64 {
        let major_val = self.major.to_i16() as f64;
        let minor_val = self.minor.to_i16() as f64;
        major_val / (1 << 14) as f64 + minor_val / (1i64 << 28) as f64
    }

    /// 1's complement negation of both words.
    pub fn negate(self) -> Self {
        DpWord {
            major: self.major.negate(),
            minor: self.minor.negate(),
        }
    }

    /// True if the sign bits of major and minor agree (well-formed DP).
    pub fn is_well_formed(&self) -> bool {
        self.major.is_positive() == self.minor.is_positive()
            || self.minor.is_zero()
            || self.major.is_zero()
    }
}

impl fmt::Debug for DpWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DpWord({:05o} {:05o} ≈ {:.10})",
            self.major.raw(),
            self.minor.raw(),
            self.to_f64()
        )
    }
}

// ---------------------------------------------------------------------------
// TpWord
// ---------------------------------------------------------------------------

impl TpWord {
    pub const ZERO: TpWord = TpWord {
        hi: AgcWord::POS_ZERO,
        mid: AgcWord::POS_ZERO,
        lo: AgcWord::POS_ZERO,
    };

    pub fn new(hi: AgcWord, mid: AgcWord, lo: AgcWord) -> Self {
        TpWord { hi, mid, lo }
    }

    /// Truncate to double precision (discard lo word).
    pub fn to_dp(self) -> DpWord {
        DpWord::new(self.hi, self.mid)
    }

    /// Extend a DpWord to triple precision with lo = +0.
    pub fn from_dp(dp: DpWord) -> Self {
        TpWord::new(dp.major, dp.minor, AgcWord::POS_ZERO)
    }
}

impl fmt::Debug for TpWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TpWord({:05o} {:05o} {:05o})",
            self.hi.raw(),
            self.mid.raw(),
            self.lo.raw()
        )
    }
}

// ---------------------------------------------------------------------------
// DAS (Double Add to Storage) — the core DP addition primitive
// ---------------------------------------------------------------------------

/// Perform DAS: add a DP value to MPAC (represented as major, minor, with overflow).
///
/// This mirrors the AGC's DAS instruction which adds the A,L register pair
/// to a DP memory location. Overflow from the minor word carries into the major.
///
/// Returns (result_major, result_minor, overflow_occurred).
pub fn das(
    dest_major: AgcWord,
    dest_minor: AgcWord,
    src_major: AgcWord,
    src_minor: AgcWord,
) -> (AgcWord, AgcWord, bool) {
    // Add minor words first
    let minor_sum = (dest_minor.to_i16() as i32) + (src_minor.to_i16() as i32);

    // Handle end-around carry for minor word
    let (minor_result, minor_carry) = ones_complement_reduce(minor_sum);

    // Add major words plus carry from minor
    let major_sum = (dest_major.to_i16() as i32) + (src_major.to_i16() as i32) + minor_carry;

    let (major_result, _) = ones_complement_reduce(major_sum);
    let overflow = major_sum > 16383 || major_sum < -16383;

    (major_result, minor_result, overflow)
}

/// Reduce a 1's complement sum to 15 bits with end-around carry.
/// Returns (reduced_word, carry_value) where carry is +1, 0, or -1.
fn ones_complement_reduce(sum: i32) -> (AgcWord, i32) {
    if sum > 16383 {
        // Positive overflow: end-around carry adds 1
        let reduced = sum - 16384 + 1; // subtract 2^14, add carry
        (AgcWord::from_i16(reduced.clamp(-16383, 16383) as i16), 1)
    } else if sum < -16383 {
        // Negative overflow: end-around borrow subtracts 1
        let reduced = sum + 16384 - 1;
        (AgcWord::from_i16(reduced.clamp(-16383, 16383) as i16), -1)
    } else {
        (AgcWord::from_i16(sum as i16), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_zero_neg_zero_distinct() {
        assert_ne!(AgcWord::POS_ZERO, AgcWord::NEG_ZERO);
        assert!(AgcWord::POS_ZERO.is_zero());
        assert!(AgcWord::NEG_ZERO.is_zero());
        assert!(AgcWord::POS_ZERO.is_pos_zero());
        assert!(AgcWord::NEG_ZERO.is_neg_zero());
    }

    #[test]
    fn test_from_i16_roundtrip() {
        for val in [-16383, -100, -1, 0, 1, 100, 16383] {
            assert_eq!(AgcWord::from_i16(val).to_i16(), val);
        }
    }

    #[test]
    fn test_negation() {
        let five = AgcWord::from_i16(5);
        assert_eq!(five.negate().to_i16(), -5);
        assert_eq!(five.negate().negate(), five);

        // +0 negates to -0
        assert_eq!(AgcWord::POS_ZERO.negate(), AgcWord::NEG_ZERO);
        assert_eq!(AgcWord::NEG_ZERO.negate(), AgcWord::POS_ZERO);
    }

    #[test]
    fn test_ccs_branches() {
        // Positive: branch 0, diminished value
        let (branch, dabs) = AgcWord::from_i16(5).ccs();
        assert_eq!(branch, 0);
        assert_eq!(dabs.to_i16(), 4);

        // +0: branch 1, result +0
        let (branch, dabs) = AgcWord::POS_ZERO.ccs();
        assert_eq!(branch, 1);
        assert!(dabs.is_pos_zero());

        // Negative: branch 2, diminished magnitude
        let (branch, dabs) = AgcWord::from_i16(-5).ccs();
        assert_eq!(branch, 2);
        assert_eq!(dabs.to_i16(), 4);

        // -0: branch 3, result +0
        let (branch, dabs) = AgcWord::NEG_ZERO.ccs();
        assert_eq!(branch, 3);
        assert!(dabs.is_pos_zero());
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(AgcWord::from_i16(100).magnitude(), 100);
        assert_eq!(AgcWord::from_i16(-100).magnitude(), 100);
        assert_eq!(AgcWord::POS_ZERO.magnitude(), 0);
        assert_eq!(AgcWord::NEG_ZERO.magnitude(), 0);
    }

    #[test]
    fn test_octal_constants() {
        // POSMAX = octal 37777 = 16383
        assert_eq!(AgcWord::from_octal(0o37777), AgcWord::POS_MAX);
        assert_eq!(AgcWord::POS_MAX.to_i16(), 16383);

        // NEG_ZERO = octal 77777
        assert_eq!(AgcWord::from_octal(0o77777), AgcWord::NEG_ZERO);
    }

    #[test]
    fn test_dp_from_f64() {
        let half = DpWord::from_f64(0.5);
        assert!((half.to_f64() - 0.5).abs() < 1e-8);

        let neg_quarter = DpWord::from_f64(-0.25);
        assert!((neg_quarter.to_f64() + 0.25).abs() < 1e-8);
    }

    #[test]
    fn test_das_basic() {
        let a = AgcWord::from_i16(100);
        let b = AgcWord::from_i16(200);
        let zero = AgcWord::POS_ZERO;

        // Simple SP add via DAS with zero minor words
        let (maj, _min, ovf) = das(a, zero, b, zero);
        assert_eq!(maj.to_i16(), 300);
        assert!(!ovf);
    }

    #[test]
    fn test_das_overflow() {
        let big = AgcWord::POS_MAX;
        let one = AgcWord::from_i16(1);
        let zero = AgcWord::POS_ZERO;

        let (_, _, ovf) = das(big, zero, one, zero);
        assert!(ovf);
    }
}
