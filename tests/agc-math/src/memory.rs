//! AGC erasable memory model with correct bank-switching.
//!
//! The Block II AGC has 2048 words of erasable (RAM) memory:
//! - Unswitched: 256 words at addresses 0000-0377 octal (always accessible)
//! - Switched: 8 banks of 256 words each, selected by the EBANK register
//!
//! The unswitched region contains critical system state: the Executive's
//! core sets, the phase/restart table, FAILREG, REDOCTR, and interrupt
//! temporaries. Switched banks hold subsystem-specific data (waitlist,
//! state vectors, W-matrix, etc.).
//!
//! ## Address Decoding
//!
//! Virtual addresses 0000-0377 always map to unswitched erasable.
//! Virtual addresses 0400-0777 map to the currently selected EBANK.
//! The physical address is: EBANK * 256 + (virtual_addr - 0400).
//!
//! ## Named Locations (from ERASABLE_ASSIGNMENTS.agc)
//!
//! All addresses are in octal, matching the AGC source conventions.

use crate::ones_complement::AgcWord;

/// Number of unswitched erasable registers.
pub const UNSWITCHED_SIZE: usize = 256; // 0000-0377 octal

/// Number of registers per switched bank.
pub const BANK_SIZE: usize = 256; // 0400-0777 virtual

/// Number of erasable banks.
pub const NUM_EBANKS: usize = 8;

/// Total erasable memory words.
pub const TOTAL_ERASABLE: usize = UNSWITCHED_SIZE + NUM_EBANKS * BANK_SIZE;

// ---------------------------------------------------------------------------
// Named addresses — unswitched erasable (0000-0377 octal)
// Reference: Comanche055/ERASABLE_ASSIGNMENTS.agc
// ---------------------------------------------------------------------------

/// Multi-purpose accumulator (7 words: MPAC through MPAC+6)
pub const ADDR_MPAC: u16 = 0o100;

/// Interpreter mode register (0=DP, +1=TP, -1=vector)
pub const ADDR_MODE: u16 = 0o107;

/// Job location counter
pub const ADDR_LOC: u16 = 0o110;

/// Bank setting register
pub const ADDR_BANKSET: u16 = 0o111;

/// Push-down pointer for interpretive programs
pub const ADDR_PUSHLOC: u16 = 0o112;

/// First PRIORITY register (core set 0). 7 core sets × 12 registers = 84 words.
pub const ADDR_PRIORITY: u16 = 0o113;

/// Number of registers per core set
pub const CORE_SET_SIZE: u16 = 12;

/// Number of core sets (Comanche=6, Luminary=7)
pub const NUM_CORE_SETS: u16 = 7;

/// Alarm code registers (3 words)
pub const ADDR_FAILREG: u16 = 0o317;

/// Phase table entries (6 phases × 2 words each: phase + complement)
pub const ADDR_PHASE1: u16 = 0o340;
pub const ADDR_PHASE2: u16 = 0o342;
pub const ADDR_PHASE3: u16 = 0o344;
pub const ADDR_PHASE4: u16 = 0o346;
pub const ADDR_PHASE5: u16 = 0o350;
pub const ADDR_PHASE6: u16 = 0o352;

/// Phase complement entries (-PHASEn = complement of PHASEn)
pub const ADDR_NEG_PHASE1: u16 = 0o341;
pub const ADDR_NEG_PHASE2: u16 = 0o343;
pub const ADDR_NEG_PHASE3: u16 = 0o345;
pub const ADDR_NEG_PHASE4: u16 = 0o347;
pub const ADDR_NEG_PHASE5: u16 = 0o351;
pub const ADDR_NEG_PHASE6: u16 = 0o353;

/// Phase priority/dispatch table (interleaved with TBASE)
pub const ADDR_PHSPRDT1: u16 = 0o363;
pub const ADDR_PHSPRDT2: u16 = 0o365;
pub const ADDR_PHSPRDT3: u16 = 0o367;
pub const ADDR_PHSPRDT4: u16 = 0o371;
pub const ADDR_PHSPRDT5: u16 = 0o373;
pub const ADDR_PHSPRDT6: u16 = 0o375;

/// Restart counter — increments on every GOPROG (hardware restart)
pub const ADDR_REDOCTR: u16 = 0o376;

// ---------------------------------------------------------------------------
// Named addresses — EBANK 3 switched erasable (virtual 0400-0777)
// Physical: 1400-1777 octal when EBANK=3
// ---------------------------------------------------------------------------

/// Waitlist delta-T entries (8 words). Virtual address in EBANK 3.
pub const ADDR_LST1: u16 = 0o400; // physical 1400 in EBANK 3

/// Waitlist task 2CADR entries (18 words). Virtual address in EBANK 3.
pub const ADDR_LST2: u16 = 0o410; // physical 1410 in EBANK 3

/// EBANK number where the waitlist lives.
pub const EBANK_WAITLIST: u8 = 3;

// ---------------------------------------------------------------------------
// Convenience: all phase table addresses as an array
// ---------------------------------------------------------------------------

/// All 6 phase addresses (for iteration).
pub const PHASE_ADDRS: [u16; 6] = [
    ADDR_PHASE1, ADDR_PHASE2, ADDR_PHASE3,
    ADDR_PHASE4, ADDR_PHASE5, ADDR_PHASE6,
];

/// All 6 phase complement addresses.
pub const NEG_PHASE_ADDRS: [u16; 6] = [
    ADDR_NEG_PHASE1, ADDR_NEG_PHASE2, ADDR_NEG_PHASE3,
    ADDR_NEG_PHASE4, ADDR_NEG_PHASE5, ADDR_NEG_PHASE6,
];

/// All 6 phase priority/dispatch addresses.
pub const PHSPRDT_ADDRS: [u16; 6] = [
    ADDR_PHSPRDT1, ADDR_PHSPRDT2, ADDR_PHSPRDT3,
    ADDR_PHSPRDT4, ADDR_PHSPRDT5, ADDR_PHSPRDT6,
];

// ---------------------------------------------------------------------------
// Memory struct
// ---------------------------------------------------------------------------

/// Model of the AGC's 2048-word erasable memory with bank switching.
#[derive(Clone)]
pub struct ErasableMemory {
    /// Unswitched erasable: addresses 0000-0377 octal (always accessible).
    unswitched: [AgcWord; UNSWITCHED_SIZE],
    /// Switched erasable: 8 banks × 256 words each.
    banks: [[AgcWord; BANK_SIZE]; NUM_EBANKS],
}

impl ErasableMemory {
    /// Create a zeroed-out erasable memory.
    pub fn new() -> Self {
        ErasableMemory {
            unswitched: [AgcWord::POS_ZERO; UNSWITCHED_SIZE],
            banks: [[AgcWord::POS_ZERO; BANK_SIZE]; NUM_EBANKS],
        }
    }

    /// Read from unswitched erasable (addr 0000-0377). Panics if out of range.
    pub fn read_unswitched(&self, addr: u16) -> AgcWord {
        assert!(addr < 0o400, "address {:05o} is not unswitched erasable", addr);
        self.unswitched[addr as usize]
    }

    /// Write to unswitched erasable (addr 0000-0377).
    pub fn write_unswitched(&mut self, addr: u16, val: AgcWord) {
        assert!(addr < 0o400, "address {:05o} is not unswitched erasable", addr);
        self.unswitched[addr as usize] = val;
    }

    /// Read from switched erasable (virtual addr 0400-0777, in the given EBANK).
    pub fn read_switched(&self, virtual_addr: u16, ebank: u8) -> AgcWord {
        assert!(
            (0o400..0o1000).contains(&virtual_addr),
            "address {:05o} is not switched erasable",
            virtual_addr
        );
        assert!(ebank < NUM_EBANKS as u8, "EBANK {} out of range", ebank);
        let offset = (virtual_addr - 0o400) as usize;
        self.banks[ebank as usize][offset]
    }

    /// Write to switched erasable.
    pub fn write_switched(&mut self, virtual_addr: u16, ebank: u8, val: AgcWord) {
        assert!(
            (0o400..0o1000).contains(&virtual_addr),
            "address {:05o} is not switched erasable",
            virtual_addr
        );
        assert!(ebank < NUM_EBANKS as u8, "EBANK {} out of range", ebank);
        let offset = (virtual_addr - 0o400) as usize;
        self.banks[ebank as usize][offset] = val;
    }

    /// Read with automatic address decoding.
    /// Addresses 0000-0377 → unswitched, 0400-0777 → switched (needs ebank).
    pub fn read(&self, addr: u16, ebank: u8) -> AgcWord {
        if addr < 0o400 {
            self.read_unswitched(addr)
        } else {
            self.read_switched(addr, ebank)
        }
    }

    /// Write with automatic address decoding.
    pub fn write(&mut self, addr: u16, ebank: u8, val: AgcWord) {
        if addr < 0o400 {
            self.write_unswitched(addr, val);
        } else {
            self.write_switched(addr, ebank, val);
        }
    }

    /// Take a snapshot of all unswitched erasable as raw u16 values.
    pub fn snapshot_unswitched(&self) -> Vec<u16> {
        self.unswitched.iter().map(|w| w.raw()).collect()
    }

    /// Take a snapshot of a specific EBANK as raw u16 values.
    pub fn snapshot_bank(&self, ebank: u8) -> Vec<u16> {
        self.banks[ebank as usize].iter().map(|w| w.raw()).collect()
    }
}

impl Default for ErasableMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unswitched_read_write() {
        let mut mem = ErasableMemory::new();
        let val = AgcWord::from_i16(1234);
        mem.write_unswitched(ADDR_REDOCTR, val);
        assert_eq!(mem.read_unswitched(ADDR_REDOCTR), val);
    }

    #[test]
    fn test_switched_bank_isolation() {
        let mut mem = ErasableMemory::new();
        let val3 = AgcWord::from_i16(333);
        let val5 = AgcWord::from_i16(555);

        mem.write_switched(ADDR_LST1, 3, val3);
        mem.write_switched(ADDR_LST1, 5, val5);

        assert_eq!(mem.read_switched(ADDR_LST1, 3), val3);
        assert_eq!(mem.read_switched(ADDR_LST1, 5), val5);
        assert_ne!(mem.read_switched(ADDR_LST1, 3), mem.read_switched(ADDR_LST1, 5));
    }

    #[test]
    fn test_auto_decode() {
        let mut mem = ErasableMemory::new();
        let val = AgcWord::from_i16(42);

        // Unswitched address
        mem.write(ADDR_FAILREG, 0, val);
        assert_eq!(mem.read(ADDR_FAILREG, 0), val);

        // Switched address
        mem.write(ADDR_LST1, EBANK_WAITLIST, val);
        assert_eq!(mem.read(ADDR_LST1, EBANK_WAITLIST), val);
    }

    #[test]
    fn test_phase_table_layout() {
        // Verify phase addresses are correctly spaced (every 2 words)
        for i in 0..5 {
            assert_eq!(PHASE_ADDRS[i + 1] - PHASE_ADDRS[i], 2);
        }
    }

    #[test]
    fn test_snapshot() {
        let mut mem = ErasableMemory::new();
        mem.write_unswitched(0o100, AgcWord::from_i16(7777));
        let snap = mem.snapshot_unswitched();
        assert_eq!(snap[0o100], AgcWord::from_i16(7777).raw());
    }
}
