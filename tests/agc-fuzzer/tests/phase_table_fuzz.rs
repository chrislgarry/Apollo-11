//! Fuzz test: Phase table corruption.
//!
//! Corrupts PHASE1-6 and -PHASE1-6 registers with random values
//! and verifies the AGC recovers (alarm 1107, then DOFSTART).
//!
//! Run with: cargo test --test phase_table_fuzz -- --ignored

use agc_fuzzer::corruption::CorruptionSet;
use agc_fuzzer::restart_oracle::RestartExpectation;
use agc_fuzzer::scenario::{run_fuzz_scenario, validate_result, FuzzConfig};
use agc_integration::agc_runner::find_yaagc;
use std::time::Duration;

fn make_config(port: u16) -> Option<FuzzConfig> {
    let yaagc = find_yaagc()?;
    let bin = "../../Comanche055/MAIN.agc.bin".to_string();
    if !std::path::Path::new(&bin).exists() {
        return None;
    }
    Some(FuzzConfig {
        yaagc_path: yaagc,
        bin_path: bin,
        port,
        boot_timeout: Duration::from_secs(5),
    })
}

#[test]
#[ignore = "requires yaAGC and assembled .bin"]
fn test_phase_table_all_zeros() {
    let config = make_config(19730).expect("yaAGC or .bin not found");
    let expectation = RestartExpectation::phase_table_corrupted();
    let corruption = CorruptionSet::phase_table(&[0; 12]);

    let result = run_fuzz_scenario(&config, &corruption, &expectation);
    validate_result(&result, &expectation).unwrap();
}

#[test]
#[ignore = "requires yaAGC and assembled .bin"]
fn test_phase_table_all_posmax() {
    let config = make_config(19731).expect("yaAGC or .bin not found");
    let expectation = RestartExpectation::phase_table_corrupted();
    let corruption = CorruptionSet::phase_table(&[0o37777; 12]);

    let result = run_fuzz_scenario(&config, &corruption, &expectation);
    validate_result(&result, &expectation).unwrap();
}

#[test]
#[ignore = "requires yaAGC and assembled .bin"]
fn test_phase_table_random_pattern() {
    let config = make_config(19732).expect("yaAGC or .bin not found");
    let expectation = RestartExpectation::phase_table_corrupted();
    // Deterministic "random" pattern for CI reproducibility
    let values: [u16; 12] = [
        0o12345, 0o54321, 0o77777, 0o00001, 0o40000, 0o37776,
        0o25252, 0o52525, 0o10101, 0o67676, 0o33333, 0o44444,
    ];
    let corruption = CorruptionSet::phase_table(&values);

    let result = run_fuzz_scenario(&config, &corruption, &expectation);
    validate_result(&result, &expectation).unwrap();
}
