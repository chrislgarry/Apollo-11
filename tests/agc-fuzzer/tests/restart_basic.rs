//! Baseline test: clean GOJAM without corruption.
//!
//! This validates the fuzzer infrastructure by verifying that a clean
//! restart (no memory corruption) works correctly. If this fails,
//! all other fuzz tests are invalid.
//!
//! Run with: cargo test --test restart_basic -- --ignored

use agc_fuzzer::corruption::{CorruptionSet, CorruptionTarget};
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
fn test_clean_restart_baseline() {
    let config = make_config(19720).expect("yaAGC or .bin not found");
    let expectation = RestartExpectation::clean_restart();

    // Empty corruption set — tests the infrastructure itself
    let corruption = CorruptionSet {
        corruptions: vec![],
        target: CorruptionTarget::RandomUnswitched,
    };

    let result = run_fuzz_scenario(&config, &corruption, &expectation);
    validate_result(&result, &expectation).unwrap();
}
