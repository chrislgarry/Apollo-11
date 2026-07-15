//! Integration test: AGC fresh start behavior.
//!
//! Verifies that a freshly booted AGC (with either Comanche055 or Luminary099)
//! initializes correctly: PROG display shows 00, no PROG ALARM light.
//!
//! Requires yaAGC and an assembled .bin file. Run with:
//!   cargo test --test fresh_start -- --ignored

use agc_integration::agc_runner::{find_yaagc, AgcRunner};
use agc_integration::dsky_client::DskyClient;
use std::time::Duration;

fn find_bin(program: &str) -> Option<String> {
    let path = format!("../../{}/MAIN.agc.bin", program);
    if std::path::Path::new(&path).exists() {
        Some(path)
    } else {
        None
    }
}

#[test]
#[ignore = "requires yaAGC and assembled .bin"]
fn test_comanche055_boots_to_p00() {
    let yaagc = find_yaagc().expect("yaAGC not found on PATH");
    let bin = find_bin("Comanche055").expect("Comanche055/MAIN.agc.bin not found");

    let mut runner = AgcRunner::start(&yaagc, &bin, Some(19701)).unwrap();
    assert!(runner.is_running(), "yaAGC failed to start");

    let mut dsky = DskyClient::connect(&runner.address()).unwrap();

    // Wait for the AGC to boot (should display PROG 00)
    let booted = dsky
        .wait_for_display(
            |d| !d.relay_words.is_empty(),
            Duration::from_secs(5),
        )
        .unwrap();

    assert!(booted, "AGC did not produce display output within 5 seconds");
    assert!(
        !dsky.display().prog_alarm,
        "PROG ALARM should not be set after fresh start"
    );

    runner.stop().unwrap();
}

#[test]
#[ignore = "requires yaAGC and assembled .bin"]
fn test_luminary099_boots_to_p00() {
    let yaagc = find_yaagc().expect("yaAGC not found on PATH");
    let bin = find_bin("Luminary099").expect("Luminary099/MAIN.agc.bin not found");

    let mut runner = AgcRunner::start(&yaagc, &bin, Some(19702)).unwrap();
    assert!(runner.is_running(), "yaAGC failed to start");

    let mut dsky = DskyClient::connect(&runner.address()).unwrap();

    let booted = dsky
        .wait_for_display(
            |d| !d.relay_words.is_empty(),
            Duration::from_secs(5),
        )
        .unwrap();

    assert!(booted, "AGC did not produce display output within 5 seconds");
    assert!(
        !dsky.display().prog_alarm,
        "PROG ALARM should not be set after fresh start"
    );

    runner.stop().unwrap();
}
