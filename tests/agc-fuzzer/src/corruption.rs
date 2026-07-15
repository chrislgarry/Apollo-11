//! Corruption strategies for fuzz testing AGC restart recovery.
//!
//! Each strategy targets a specific subsystem's erasable memory,
//! generating random values that would break the subsystem's invariants.

use crate::memory_map::*;

/// What subsystem the corruption targets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CorruptionTarget {
    /// PHASE1-6 and -PHASE1-6: restart protection table.
    /// A corrupted phase table should trigger alarm 1107 on restart.
    PhaseTable,

    /// PHSPRDT1-6: phase priority/dispatch entries.
    /// Controls whether restarts execute as FINDVAC, NOVAC, or LONGCALL.
    PhasePriority,

    /// FAILREG, FAILREG+1, FAILREG+2: alarm code registers.
    /// Should be overwritten with the restart alarm code on GOPROG.
    FailReg,

    /// PRIORITY registers in the 7 Executive core sets.
    /// Corrupt priorities can cause EXECUTIVE to mis-schedule or deadlock.
    Executive,

    /// LST1 (delta-T) and LST2 (2CADR) in EBANK 3.
    /// Corrupted timing values can cause waitlist to fire incorrectly.
    Waitlist,

    /// Any address in unswitched erasable (0000-0377).
    /// Tests that FRESH_START_AND_RESTART handles arbitrary corruption.
    RandomUnswitched,
}

/// A single corruption: one word written to one address.
#[derive(Clone, Debug)]
pub struct Corruption {
    /// Which subsystem this targets.
    pub target: CorruptionTarget,
    /// The erasable address to corrupt (octal).
    pub address: u16,
    /// The value to write (raw 15-bit AGC word).
    pub value: u16,
    /// EBANK selection (only relevant for switched addresses).
    pub ebank: u8,
}

/// A set of corruptions to apply simultaneously before triggering GOJAM.
#[derive(Clone, Debug)]
pub struct CorruptionSet {
    pub corruptions: Vec<Corruption>,
    pub target: CorruptionTarget,
}

impl CorruptionSet {
    /// Generate a phase table corruption: random values in all 12 phase registers.
    pub fn phase_table(values: &[u16; 12]) -> Self {
        let mut corruptions = Vec::with_capacity(12);
        for (i, &val) in values.iter().enumerate() {
            let addr = if i < 6 {
                PHASE_ADDRS[i]
            } else {
                NEG_PHASE_ADDRS[i - 6]
            };
            corruptions.push(Corruption {
                target: CorruptionTarget::PhaseTable,
                address: addr,
                value: val & 0x7FFF, // mask to 15 bits
                ebank: 0,
            });
        }
        CorruptionSet {
            corruptions,
            target: CorruptionTarget::PhaseTable,
        }
    }

    /// Generate a phase priority corruption.
    pub fn phase_priority(values: &[u16; 6]) -> Self {
        let corruptions = PHSPRDT_ADDRS
            .iter()
            .zip(values.iter())
            .map(|(&addr, &val)| Corruption {
                target: CorruptionTarget::PhasePriority,
                address: addr,
                value: val & 0x7FFF,
                ebank: 0,
            })
            .collect();
        CorruptionSet {
            corruptions,
            target: CorruptionTarget::PhasePriority,
        }
    }

    /// Generate FAILREG corruption (3 words).
    pub fn failreg(values: [u16; 3]) -> Self {
        let corruptions = (0..3)
            .map(|i| Corruption {
                target: CorruptionTarget::FailReg,
                address: ADDR_FAILREG + i,
                value: values[i as usize] & 0x7FFF,
                ebank: 0,
            })
            .collect();
        CorruptionSet {
            corruptions,
            target: CorruptionTarget::FailReg,
        }
    }

    /// Generate executive corruption: random priorities for N core sets.
    pub fn executive(priorities: &[u16]) -> Self {
        let corruptions = priorities
            .iter()
            .enumerate()
            .map(|(i, &val)| Corruption {
                target: CorruptionTarget::Executive,
                address: core_set_priority_addr(i as u16),
                value: val & 0x7FFF,
                ebank: 0,
            })
            .collect();
        CorruptionSet {
            corruptions,
            target: CorruptionTarget::Executive,
        }
    }

    /// Generate waitlist corruption: random delta-T values in LST1.
    pub fn waitlist_timing(deltas: &[u16; 8]) -> Self {
        let corruptions = deltas
            .iter()
            .enumerate()
            .map(|(i, &val)| Corruption {
                target: CorruptionTarget::Waitlist,
                address: ADDR_LST1 + i as u16,
                value: val & 0x7FFF,
                ebank: EBANK_WAITLIST,
            })
            .collect();
        CorruptionSet {
            corruptions,
            target: CorruptionTarget::Waitlist,
        }
    }

    /// Generate a single random unswitched erasable corruption.
    pub fn random_single(address: u16, value: u16) -> Self {
        assert!(address <= MAX_UNSWITCHED);
        CorruptionSet {
            corruptions: vec![Corruption {
                target: CorruptionTarget::RandomUnswitched,
                address,
                value: value & 0x7FFF,
                ebank: 0,
            }],
            target: CorruptionTarget::RandomUnswitched,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_table_corruption_addresses() {
        let values = [0o12345; 12];
        let cs = CorruptionSet::phase_table(&values);
        assert_eq!(cs.corruptions.len(), 12);
        assert_eq!(cs.corruptions[0].address, ADDR_PHASE1);
        assert_eq!(cs.corruptions[5].address, ADDR_PHASE6);
        assert_eq!(cs.corruptions[6].address, ADDR_NEG_PHASE1);
        assert_eq!(cs.corruptions[11].address, ADDR_NEG_PHASE6);
    }

    #[test]
    fn test_executive_corruption_spacing() {
        let priorities = [0o30000; 7];
        let cs = CorruptionSet::executive(&priorities);
        assert_eq!(cs.corruptions.len(), 7);
        // Core sets are 12 registers apart
        assert_eq!(
            cs.corruptions[1].address - cs.corruptions[0].address,
            CORE_SET_SIZE
        );
    }

    #[test]
    fn test_value_masked_to_15_bits() {
        let cs = CorruptionSet::random_single(0o200, 0xFFFF);
        assert_eq!(cs.corruptions[0].value, 0x7FFF);
    }
}
