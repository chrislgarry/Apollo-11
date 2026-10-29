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

/// Total erasable memory words (8 banks × 256 = 2048).
/// Unswitched erasable (0000-0377) physically IS EBANK 0 — same memory.
pub const TOTAL_ERASABLE: usize = NUM_EBANKS * BANK_SIZE;

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

/// Number of core sets (7 in both Comanche055 and Luminary099)
pub const NUM_CORE_SETS: u16 = 7;

/// Alarm code registers (3 words)
pub const ADDR_FAILREG: u16 = 0o317;

/// Phase table entries (6 phases �� 2 words each: complement then phase).
/// In ERASABLE_ASSIGNMENTS.agc, -PHASEn is allocated first (even addr),
/// then PHASEn at the next word (odd addr).
pub const ADDR_NEG_PHASE1: u16 = 0o340;
pub const ADDR_PHASE1: u16 = 0o341;
pub const ADDR_NEG_PHASE2: u16 = 0o342;
pub const ADDR_PHASE2: u16 = 0o343;
pub const ADDR_NEG_PHASE3: u16 = 0o344;
pub const ADDR_PHASE3: u16 = 0o345;
pub const ADDR_NEG_PHASE4: u16 = 0o346;
pub const ADDR_PHASE4: u16 = 0o347;
pub const ADDR_NEG_PHASE5: u16 = 0o350;
pub const ADDR_PHASE5: u16 = 0o351;
pub const ADDR_NEG_PHASE6: u16 = 0o352;
pub const ADDR_PHASE6: u16 = 0o353;

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
///
/// Unswitched erasable (0000-0377) physically IS EBANK 0 — they are the
/// same 256 words of memory. Writing to unswitched address 0o100 and
/// reading from switched address 0o500 with EBANK=0 returns the same value.
#[derive(Clone)]
pub struct ErasableMemory {
    /// 8 erasable banks × 256 words each = 2048 words total.
    /// Bank 0 is also accessible as unswitched erasable (0000-0377).
    banks: [[AgcWord; BANK_SIZE]; NUM_EBANKS],
}

impl ErasableMemory {
    /// Create a zeroed-out erasable memory.
    pub fn new() -> Self {
        ErasableMemory {
            banks: [[AgcWord::POS_ZERO; BANK_SIZE]; NUM_EBANKS],
        }
    }

    /// Read from unswitched erasable (addr 0000-0377).
    /// This reads from EBANK 0 — the physical memory is shared.
    pub fn read_unswitched(&self, addr: u16) -> AgcWord {
        assert!(addr < 0o400, "address {:05o} is not unswitched erasable", addr);
        self.banks[0][addr as usize]
    }

    /// Write to unswitched erasable (addr 0000-0377).
    /// This writes to EBANK 0 — the physical memory is shared.
    pub fn write_unswitched(&mut self, addr: u16, val: AgcWord) {
        assert!(addr < 0o400, "address {:05o} is not unswitched erasable", addr);
        self.banks[0][addr as usize] = val;
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
    /// Addresses 0000-0377 → EBANK 0 (unswitched), 0400-0777 → selected EBANK.
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

    /// Take a snapshot of unswitched erasable (EBANK 0) as raw u16 values.
    pub fn snapshot_unswitched(&self) -> Vec<u16> {
        self.banks[0].iter().map(|w| w.raw()).collect()
    }

    /// Take a snapshot of a specific EBANK as raw u16 values.
    pub fn snapshot_bank(&self, ebank: u8) -> Vec<u16> {
        assert!(ebank < NUM_EBANKS as u8, "EBANK {} out of range", ebank);
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
    fn test_ebank0_aliasing() {
        // Unswitched erasable (0000-0377) IS EBANK 0 — same physical memory.
        let mut mem = ErasableMemory::new();
        let val = AgcWord::from_i16(9999);

        // Write via unswitched path
        mem.write_unswitched(0o100, val);

        // Read via switched path with EBANK=0 — must see the same value
        assert_eq!(
            mem.read_switched(0o500, 0),
            val,
            "unswitched 0o100 and switched 0o500 with EBANK=0 must alias"
        );

        // Write via switched EBANK=0, read via unswitched
        let val2 = AgcWord::from_i16(4242);
        mem.write_switched(0o600, 0, val2);
        assert_eq!(
            mem.read_unswitched(0o200),
            val2,
            "switched 0o600 EBANK=0 and unswitched 0o200 must alias"
        );
    }

    #[test]
    fn test_boundary_addresses() {
        let mut mem = ErasableMemory::new();
        let val = AgcWord::from_i16(1111);

        // Last unswitched address
        mem.write_unswitched(0o377, val);
        assert_eq!(mem.read_unswitched(0o377), val);

        // First switched address
        mem.write_switched(0o400, 3, val);
        assert_eq!(mem.read_switched(0o400, 3), val);

        // Last switched address
        mem.write_switched(0o777, 7, val);
        assert_eq!(mem.read_switched(0o777, 7), val);

        // Auto-decode boundary: 0o377 → unswitched, 0o400 → switched
        mem.write(0o377, 0, val);
        assert_eq!(mem.read(0o377, 0), val);
        mem.write(0o400, 5, val);
        assert_eq!(mem.read(0o400, 5), val);
    }

    #[test]
    #[should_panic(expected = "not switched erasable")]
    fn test_switched_addr_overflow() {
        let mem = ErasableMemory::new();
        mem.read_switched(0o1000, 0); // past end of switched window
    }

    #[test]
    #[should_panic(expected = "EBANK 8 out of range")]
    fn test_ebank_overflow() {
        let mem = ErasableMemory::new();
        mem.read_switched(0o400, 8);
    }

    #[test]
    #[should_panic(expected = "EBANK 9 out of range")]
    fn test_snapshot_bank_overflow() {
        let mem = ErasableMemory::new();
        mem.snapshot_bank(9);
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
        // -PHASEn at even address, PHASEn at odd address (complement first)
        for i in 0..6 {
            assert_eq!(
                PHASE_ADDRS[i],
                NEG_PHASE_ADDRS[i] + 1,
                "PHASEn should be one word after -PHASEn"
            );
        }
        // Pairs spaced 2 words apart
        for i in 0..5 {
            assert_eq!(NEG_PHASE_ADDRS[i + 1] - NEG_PHASE_ADDRS[i], 2);
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
