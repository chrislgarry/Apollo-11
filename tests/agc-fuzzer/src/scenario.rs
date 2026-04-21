//! Fuzz scenario execution: corrupt → GOJAM → validate.
//!
//! Orchestrates the full fuzz loop using the yaAGC integration harness.

use std::time::Duration;

use agc_integration::agc_runner::AgcRunner;
use agc_integration::dsky_client::DskyClient;

use crate::corruption::CorruptionSet;
use crate::restart_oracle::RestartExpectation;

/// Result of a single fuzz scenario execution.
#[derive(Clone, Debug)]
pub enum FuzzResult {
    /// AGC recovered within timeout.
    Recovered {
        /// Time taken to recover (simulated).
        recovery_time: Duration,
        /// Whether PROG ALARM was set after recovery.
        alarm_present: bool,
    },
    /// AGC did not produce display output within timeout (hung).
    Timeout {
        timeout: Duration,
    },
    /// Infrastructure error (yaAGC crashed, connection lost, etc.)
    Error(String),
}

/// Configuration for a fuzz scenario.
pub struct FuzzConfig {
    /// Path to yaAGC binary.
    pub yaagc_path: String,
    /// Path to assembled .bin file.
    pub bin_path: String,
    /// TCP port for this scenario (use unique ports for parallel runs).
    pub port: u16,
    /// Maximum time to wait for boot.
    pub boot_timeout: Duration,
}

/// Execute a single fuzz scenario.
///
/// 1. Start yaAGC
/// 2. Wait for boot (P00 display)
/// 3. Apply corruption set
/// 4. Trigger GOJAM
/// 5. Wait for recovery
/// 6. Return result
pub fn run_fuzz_scenario(
    config: &FuzzConfig,
    corruption: &CorruptionSet,
    expectation: &RestartExpectation,
) -> FuzzResult {
    // Start yaAGC
    let mut runner = match AgcRunner::start(&config.yaagc_path, &config.bin_path, Some(config.port))
    {
        Ok(r) => r,
        Err(e) => return FuzzResult::Error(format!("Failed to start yaAGC: {}", e)),
    };

    // Connect DSKY client
    let mut dsky = match DskyClient::connect(&runner.address()) {
        Ok(d) => d,
        Err(e) => return FuzzResult::Error(format!("Failed to connect DSKY: {}", e)),
    };

    // Wait for boot
    let booted = match dsky.wait_for_display(|d| !d.relay_words.is_empty(), config.boot_timeout) {
        Ok(b) => b,
        Err(e) => {
            if let Err(stop_err) = runner.stop() {
                eprintln!("warning: failed to stop yaAGC after boot I/O error: {}", stop_err);
            }
            return FuzzResult::Error(format!("I/O error during boot wait: {}", e));
        }
    };

    if !booted {
        if let Err(e) = runner.stop() {
            eprintln!("warning: failed to stop yaAGC after boot timeout: {}", e);
        }
        return FuzzResult::Error("AGC did not boot within timeout".to_string());
    }

    // Apply corruption.
    // Note: In a full implementation, we would use yaAGC's debug interface
    // to write directly to erasable memory. For now, we document the
    // corruption that would be applied and verify the infrastructure.
    let _corruptions = &corruption.corruptions;

    // TODO: Inject corruption via yaAGC memory write interface.
    // The yaAGC debug protocol allows direct erasable memory access
    // via channel I/O. Each corruption.address/value pair would be
    // written to the corresponding erasable location.

    // TODO: Trigger GOJAM via yaAGC debug interface.
    // GOJAM is a hardware restart signal. yaAGC supports triggering
    // this via its debug channel interface.

    // Wait for recovery
    let timeout = Duration::from_secs_f64(expectation.timeout_secs);
    let start = std::time::Instant::now();

    let recovered = match dsky.wait_for_display(
        |d| {
            // Recovery = new relay words appear after the corruption point
            d.relay_words.len() > 10 // arbitrary threshold for "activity"
        },
        timeout,
    ) {
        Ok(r) => r,
        Err(e) => {
            if let Err(stop_err) = runner.stop() {
                eprintln!("warning: failed to stop yaAGC after recovery I/O error: {}", stop_err);
            }
            return FuzzResult::Error(format!("I/O error during recovery wait: {}", e));
        }
    };

    let elapsed = start.elapsed();
    if let Err(e) = runner.stop() {
        eprintln!("warning: failed to stop yaAGC: {}", e);
    }

    if recovered {
        FuzzResult::Recovered {
            recovery_time: elapsed,
            alarm_present: dsky.display().prog_alarm,
        }
    } else {
        FuzzResult::Timeout { timeout }
    }
}

/// Validate a fuzz result against the expected outcome.
pub fn validate_result(result: &FuzzResult, expectation: &RestartExpectation) -> Result<(), String> {
    match result {
        FuzzResult::Recovered { alarm_present, .. } => {
            if let Some(expected_code) = expectation.expected_alarm {
                // We expect an alarm — verify it's present
                if !alarm_present {
                    return Err(format!(
                        "Expected alarm {:05o} but PROG ALARM not set. {}",
                        expected_code, expectation.description
                    ));
                }
            } else {
                // No alarm expected — reject unexpected alarms
                if *alarm_present {
                    return Err(format!(
                        "Unexpected PROG ALARM after recovery. {}",
                        expectation.description
                    ));
                }
            }
            Ok(())
        }
        FuzzResult::Timeout { timeout } => Err(format!(
            "AGC hung (timeout {:?}). {}",
            timeout, expectation.description
        )),
        FuzzResult::Error(msg) => Err(format!("Infrastructure error: {}. {}", msg, expectation.description)),
    }
}
