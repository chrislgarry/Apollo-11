//! Named AGC erasable memory addresses for the fuzzer.
//!
//! Every address used by the fuzzer has a name, an octal value, and a source
//! reference. No magic numbers in the fuzzer code.
//!
//! These are re-exported from agc_math::memory for convenience, plus
//! additional fuzzer-specific groupings.

pub use agc_math::memory::{
    ADDR_FAILREG, ADDR_LOC, ADDR_LST1, ADDR_LST2, ADDR_MODE, ADDR_MPAC,
    ADDR_NEG_PHASE1, ADDR_NEG_PHASE2, ADDR_NEG_PHASE3, ADDR_NEG_PHASE4,
    ADDR_NEG_PHASE5, ADDR_NEG_PHASE6, ADDR_PHASE1, ADDR_PHASE2, ADDR_PHASE3,
    ADDR_PHASE4, ADDR_PHASE5, ADDR_PHASE6, ADDR_PHSPRDT1, ADDR_PHSPRDT2,
    ADDR_PHSPRDT3, ADDR_PHSPRDT4, ADDR_PHSPRDT5, ADDR_PHSPRDT6, ADDR_PRIORITY,
    ADDR_REDOCTR, CORE_SET_SIZE, EBANK_WAITLIST, NEG_PHASE_ADDRS,
    NUM_CORE_SETS, PHASE_ADDRS, PHSPRDT_ADDRS,
};

/// All phase-related addresses (phases + complements + priorities).
/// 18 registers total that form the restart protection table.
pub const ALL_PHASE_ADDRS: [u16; 18] = [
    ADDR_PHASE1, ADDR_NEG_PHASE1,
    ADDR_PHASE2, ADDR_NEG_PHASE2,
    ADDR_PHASE3, ADDR_NEG_PHASE3,
    ADDR_PHASE4, ADDR_NEG_PHASE4,
    ADDR_PHASE5, ADDR_NEG_PHASE5,
    ADDR_PHASE6, ADDR_NEG_PHASE6,
    ADDR_PHSPRDT1, ADDR_PHSPRDT2, ADDR_PHSPRDT3,
    ADDR_PHSPRDT4, ADDR_PHSPRDT5, ADDR_PHSPRDT6,
];

/// Executive PRIORITY addresses for all 7 core sets.
/// Core set N starts at ADDR_PRIORITY + N * CORE_SET_SIZE.
pub fn core_set_priority_addr(core_set: u16) -> u16 {
    assert!(core_set < NUM_CORE_SETS, "core set {} out of range", core_set);
    ADDR_PRIORITY + core_set * CORE_SET_SIZE
}

/// Address range for all executive core sets.
pub const EXECUTIVE_START: u16 = ADDR_PRIORITY; // 0o113
pub const EXECUTIVE_END: u16 = ADDR_PRIORITY + NUM_CORE_SETS * CORE_SET_SIZE; // 0o237

/// FAILREG spans 3 consecutive words.
pub const FAILREG_SIZE: u16 = 3;

/// Waitlist LST1 spans 8 words (delta-T entries for 8 task slots).
pub const LST1_SIZE: u16 = 8;

/// Waitlist LST2 spans 18 words (9 tasks × 2 words each for 2CADR).
pub const LST2_SIZE: u16 = 18;

/// Maximum unswitched erasable address (for random fuzzing range).
pub const MAX_UNSWITCHED: u16 = 0o377;
