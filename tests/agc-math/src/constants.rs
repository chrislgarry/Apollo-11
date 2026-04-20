//! AGC constants from FIXED_FIXED_CONSTANT_POOL.agc and INTERPRETIVE_CONSTANTS.agc.
//!
//! These are the exact values used by the flight software. The octal values are
//! transcribed directly from the source files.

use crate::ones_complement::AgcWord;

// ---------------------------------------------------------------------------
// Bit constants (FIXED_FIXED_CONSTANT_POOL.agc lines 47-66)
// ---------------------------------------------------------------------------

pub const BIT1: AgcWord = AgcWord::from_raw_const(0o00001);
pub const BIT2: AgcWord = AgcWord::from_raw_const(0o00002);
pub const BIT3: AgcWord = AgcWord::from_raw_const(0o00004);
pub const BIT4: AgcWord = AgcWord::from_raw_const(0o00010);
pub const BIT5: AgcWord = AgcWord::from_raw_const(0o00020);
pub const BIT6: AgcWord = AgcWord::from_raw_const(0o00040);
pub const BIT7: AgcWord = AgcWord::from_raw_const(0o00100);
pub const BIT8: AgcWord = AgcWord::from_raw_const(0o00200);
pub const BIT9: AgcWord = AgcWord::from_raw_const(0o00400);
pub const BIT10: AgcWord = AgcWord::from_raw_const(0o01000);
pub const BIT11: AgcWord = AgcWord::from_raw_const(0o02000);
pub const BIT12: AgcWord = AgcWord::from_raw_const(0o04000);
pub const BIT13: AgcWord = AgcWord::from_raw_const(0o10000);
pub const BIT14: AgcWord = AgcWord::from_raw_const(0o20000);

// ---------------------------------------------------------------------------
// Named constants (FIXED_FIXED_CONSTANT_POOL.agc lines 37-253)
// ---------------------------------------------------------------------------

/// POSMAX = octal 37777 = 16383 (maximum positive SP value)
pub const POSMAX: AgcWord = AgcWord::POS_MAX;

/// DPOSMAX = same as POSMAX (the "D" prefix is for assembly label alignment)
pub const DPOSMAX: AgcWord = POSMAX;

/// NEGMAX = octal 40000 = -16383 (maximum negative SP value)
pub const NEGMAX: AgcWord = AgcWord::NEG_MAX;

/// ZERO
pub const ZERO: AgcWord = AgcWord::POS_ZERO;

/// ONE = BIT1 = 1
pub const ONE: AgcWord = BIT1;

/// TWO
pub const TWO: AgcWord = AgcWord::from_raw_const(0o00002);

/// THREE
pub const THREE: AgcWord = AgcWord::from_raw_const(0o00003);

/// FOUR
pub const FOUR: AgcWord = AgcWord::from_raw_const(0o00004);

/// FIVE
pub const FIVE: AgcWord = AgcWord::from_raw_const(0o00005);

/// SIX
pub const SIX: AgcWord = AgcWord::from_raw_const(0o00006);

/// SEVEN
pub const SEVEN: AgcWord = AgcWord::from_raw_const(0o00007);

/// QUARTER = BIT13 = octal 10000 = 4096 (represents 0.25 in fractional scaling)
pub const QUARTER: AgcWord = BIT13;

/// HALF = BIT14 = octal 20000 = 8192 (represents 0.5 in fractional scaling)
pub const HALF: AgcWord = BIT14;

/// NEG1/2 = octal -20000 (1's complement of HALF) = -8192
/// Used by the SINE routine. Must be two locations before BIT14.
pub const NEG_HALF: AgcWord = AgcWord::from_raw_const(0o57777);

/// LOW7 = octal 00177 = 127 (mask for low 7 bits)
pub const LOW7: AgcWord = AgcWord::from_raw_const(0o00177);

/// -BIT14 = octal 57777 (1's complement negation of BIT14)
pub const NEG_BIT14: AgcWord = AgcWord::from_raw_const(0o57777);
