//! Restart oracle: defines expected recovery behavior for each corruption target.
//!
//! After a GOJAM (hardware restart), the AGC's FRESH_START_AND_RESTART routine
//! must recover to a safe state. The oracle defines what "recovered correctly"
//! means for each type of corruption, based on the actual restart logic in
//! Comanche055/FRESH_START_AND_RESTART.agc.

use crate::corruption::CorruptionTarget;
use crate::memory_map::ADDR_REDOCTR;

/// The expected outcome after a GOJAM with a given corruption.
#[derive(Clone, Debug)]
pub struct RestartExpectation {
    /// The corruption target that was applied.
    pub target: CorruptionTarget,
    /// Whether an alarm code is expected in the DSKY PROG display.
    pub expected_alarm: Option<u16>,
    /// Whether REDOCTR should increment.
    pub redoctr_increments: bool,
    /// Maximum time (in seconds of simulated AGC time) before the restart
    /// must complete. If exceeded, the AGC is considered hung.
    pub timeout_secs: f64,
    /// Description of what should happen (for test failure messages).
    pub description: &'static str,
}

impl RestartExpectation {
    /// Expected behavior when phase table is corrupted.
    ///
    /// GOPROG3 (line ~468) validates phase tables. Invalid phases trigger
    /// alarm 1107 and fall through to DOFSTART, which reinitializes everything.
    pub fn phase_table_corrupted() -> Self {
        RestartExpectation {
            target: CorruptionTarget::PhaseTable,
            expected_alarm: Some(0o1107),
            redoctr_increments: true,
            timeout_secs: 5.0,
            description: "Phase table corruption → alarm 1107, DOFSTART recovery",
        }
    }

    /// Expected behavior when phase priority is corrupted.
    ///
    /// PHSPRDT corruption may cause incorrect job types during restart,
    /// but should not hang. The AGC should still reach a stable state.
    pub fn phase_priority_corrupted() -> Self {
        RestartExpectation {
            target: CorruptionTarget::PhasePriority,
            expected_alarm: None, // may or may not alarm depending on values
            redoctr_increments: true,
            timeout_secs: 5.0,
            description: "Phase priority corruption → restart completes without hang",
        }
    }

    /// Expected behavior when FAILREG is corrupted.
    ///
    /// FAILREG is preserved through restarts (it's "permanent erasable"),
    /// but GOPROG will write its own alarm code. After restart, FAILREG
    /// should contain the restart-related alarm, not the corrupted value.
    pub fn failreg_corrupted() -> Self {
        RestartExpectation {
            target: CorruptionTarget::FailReg,
            expected_alarm: None,
            redoctr_increments: true,
            timeout_secs: 5.0,
            description: "FAILREG corruption → overwritten with restart alarm code",
        }
    }

    /// Expected behavior when Executive core sets are corrupted.
    ///
    /// STARTSUB/STARTSB2 reinitializes the Executive during restart.
    /// Corrupt priorities should be cleared.
    pub fn executive_corrupted() -> Self {
        RestartExpectation {
            target: CorruptionTarget::Executive,
            expected_alarm: None,
            redoctr_increments: true,
            timeout_secs: 5.0,
            description: "Executive corruption → core sets reinitialized via STARTSUB",
        }
    }

    /// Expected behavior when Waitlist is corrupted.
    ///
    /// STARTSUB reinitializes LST1/LST2 to ENDTASK and NEG1/2 values.
    pub fn waitlist_corrupted() -> Self {
        RestartExpectation {
            target: CorruptionTarget::Waitlist,
            expected_alarm: None,
            redoctr_increments: true,
            timeout_secs: 5.0,
            description: "Waitlist corruption → LST1/LST2 reinitialized",
        }
    }

    /// Expected behavior for arbitrary erasable corruption.
    ///
    /// The only hard requirement: the AGC must not hang. REDOCTR must
    /// increment (proving GOPROG executed), and the AGC must produce
    /// display output within the timeout (proving it's running).
    pub fn random_corruption() -> Self {
        RestartExpectation {
            target: CorruptionTarget::RandomUnswitched,
            expected_alarm: None, // unpredictable
            redoctr_increments: true,
            timeout_secs: 10.0, // longer timeout for unknown corruption
            description: "Random corruption → AGC recovers without hanging",
        }
    }

    /// Baseline: clean GOJAM with no corruption.
    ///
    /// This validates the fuzzer infrastructure itself. A clean restart
    /// should increment REDOCTR and return to P00 without any alarm.
    pub fn clean_restart() -> Self {
        RestartExpectation {
            target: CorruptionTarget::RandomUnswitched, // placeholder
            expected_alarm: None,
            redoctr_increments: true,
            timeout_secs: 3.0,
            description: "Clean GOJAM → normal restart, REDOCTR increments, no alarm",
        }
    }
}

/// Address of the restart counter for validation.
pub const REDOCTR_ADDR: u16 = ADDR_REDOCTR;
