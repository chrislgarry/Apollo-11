//! Fuzz test: Random erasable memory corruption.
//!
//! Corrupts arbitrary unswitched erasable addresses and verifies the AGC
//! does not hang. This is the most general fuzz test — it exercises
//! FRESH_START_AND_RESTART's ability to recover from any corruption.
//!
//! Run with: cargo test --test general_erasable -- --ignored

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
fn test_corrupt_mpac() {
    let config = make_config(19740).expect("yaAGC or .bin not found");
    let expectation = RestartExpectation::random_corruption();
    let corruption = CorruptionSet::random_single(0o100, 0o77777); // MPAC = NEGZERO

    let result = run_fuzz_scenario(&config, &corruption, &expectation);
    validate_result(&result, &expectation).unwrap();
}

#[test]
#[ignore = "requires yaAGC and assembled .bin"]
fn test_corrupt_newjob() {
    let config = make_config(19741).expect("yaAGC or .bin not found");
    let expectation = RestartExpectation::random_corruption();
    let corruption = CorruptionSet::random_single(0o067, 0o37777); // NEWJOB = POSMAX

    let result = run_fuzz_scenario(&config, &corruption, &expectation);
    validate_result(&result, &expectation).unwrap();
}

#[test]
#[ignore = "requires yaAGC and assembled .bin"]
fn test_corrupt_state_flagwords() {
    let config = make_config(19742).expect("yaAGC or .bin not found");
    let expectation = RestartExpectation::random_corruption();
    // STATE/FLAGWRD0 at address 0000
    let corruption = CorruptionSet::random_single(0o000, 0o77777);

    let result = run_fuzz_scenario(&config, &corruption, &expectation);
    validate_result(&result, &expectation).unwrap();
}
