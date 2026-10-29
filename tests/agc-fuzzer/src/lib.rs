//! Restart/recovery fuzzer for the Apollo Guidance Computer.
//!
//! This crate tests the AGC's FRESH_START_AND_RESTART recovery mechanism by:
//! 1. Running the AGC in yaAGC to a known good state
//! 2. Injecting corruption into specific erasable memory locations
//! 3. Triggering a GOJAM (hardware restart)
//! 4. Verifying that FRESH_START_AND_RESTART recovers correctly
//!
//! The AGC's restart architecture (designed by Margaret Hamilton's team) was
//! built to survive arbitrary memory corruption and continue the mission.
//! The 1201/1202 alarms during Apollo 11's landing proved this design worked
//! under real conditions. This fuzzer systematically validates that resilience.
//!
//! ## Corruption Targets
//!
//! - Phase table (PHASE1-6): Controls which programs restart after GOJAM
//! - FAILREG: Alarm code storage (should be overwritten on restart)
//! - Executive core sets (PRIORITY): Job scheduling state
//! - Waitlist (LST1/LST2): Timer-based task queue
//! - General erasable: Any address in unswitched memory

pub mod memory_map;
pub mod corruption;
pub mod restart_oracle;
pub mod scenario;
