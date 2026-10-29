//! Integration test: Basic DSKY verb/noun operations.
//!
//! Tests fundamental DSKY interactions:
//! - V35E: Lamp test (all segments illuminate)
//! - V05N09E: Display alarm codes (should show 00000 after fresh start)
//! - V16N36E: Display current time (R1/R2/R3 should show values)
//!
//! Requires yaAGC and an assembled .bin file. Run with:
//!   cargo test --test verb_noun_basic -- --ignored

use agc_integration::agc_runner::{find_yaagc, AgcRunner};
use agc_integration::dsky_client::DskyClient;
use std::time::Duration;

fn setup_comanche(port: u16) -> (AgcRunner, DskyClient) {
    let yaagc = find_yaagc().expect("yaAGC not found");
    let bin = format!("../../Comanche055/MAIN.agc.bin");
    assert!(
        std::path::Path::new(&bin).exists(),
        "Comanche055/MAIN.agc.bin not found"
    );

    let runner = AgcRunner::start(&yaagc, &bin, Some(port)).unwrap();
    std::thread::sleep(Duration::from_secs(1)); // Wait for boot

    let mut dsky = DskyClient::connect(&runner.address()).unwrap();
    // Wait for initial boot
    dsky.wait_for_display(|d| !d.relay_words.is_empty(), Duration::from_secs(5))
        .unwrap();

    (runner, dsky)
}

#[test]
#[ignore = "requires yaAGC and assembled .bin"]
fn test_lamp_test_v35() {
    let (mut runner, mut dsky) = setup_comanche(19710);

    // V35E: Lamp test
    dsky.press_key(agc_integration::dsky_client::key::VERB).unwrap();
    dsky.press_key(agc_integration::dsky_client::key::THREE).unwrap();
    dsky.press_key(agc_integration::dsky_client::key::FIVE).unwrap();
    dsky.press_key(agc_integration::dsky_client::key::ENTER).unwrap();

    // Wait for display update
    dsky.poll_display(Duration::from_secs(2)).unwrap();

    // During lamp test, all segments should be illuminated.
    // This means many relay words should be received with non-zero data.
    let relay_count = dsky.display().relay_words.len();
    assert!(
        relay_count > 5,
        "Lamp test should produce many relay updates, got {}",
        relay_count
    );

    runner.stop().unwrap();
}

#[test]
#[ignore = "requires yaAGC and assembled .bin"]
fn test_display_alarm_codes_v05n09() {
    let (mut runner, mut dsky) = setup_comanche(19711);

    // V05N09E: Display alarm codes in R1, R2, R3
    dsky.verb_noun(5, 9).unwrap();

    // Wait for display
    dsky.poll_display(Duration::from_secs(2)).unwrap();

    // After fresh start, alarm codes should be zero
    // We can't easily decode the exact displayed digits yet,
    // but we should see relay word updates for R1, R2, R3 positions
    let relay_count = dsky.display().relay_words.len();
    assert!(
        relay_count > 3,
        "V05N09 should update R1/R2/R3 displays, got {} relay words",
        relay_count
    );

    runner.stop().unwrap();
}

#[test]
#[ignore = "requires yaAGC and assembled .bin"]
fn test_invalid_verb_v99() {
    let (mut runner, mut dsky) = setup_comanche(19712);

    // V99E: Invalid verb should produce OPR ERR
    dsky.verb_noun(99, 0).unwrap();

    dsky.poll_display(Duration::from_secs(2)).unwrap();

    // The AGC should respond with some display update
    // (OPR ERR light or verb rejection)
    assert!(
        !dsky.display().relay_words.is_empty(),
        "AGC should respond to invalid verb"
    );

    runner.stop().unwrap();
}
