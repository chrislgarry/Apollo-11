//! Integration test harness for the Apollo Guidance Computer.
//!
//! This crate provides infrastructure for running assembled AGC binaries
//! in the yaAGC simulator and verifying behavior by acting as a virtual
//! DSKY (Display/Keyboard) peripheral over TCP.
//!
//! ## Architecture
//!
//! yaAGC communicates with peripherals using a simple 4-byte packet protocol
//! over TCP sockets. Each packet encodes a channel number and a 15-bit value.
//! The DSKY interface uses:
//! - Channel 010 (octal): Relay word output (display digit data to DSKY)
//! - Channel 011 (octal): Discrete alarm/status outputs (DSALMOUT)
//! - Channel 013 (octal): Discrete outputs (warning lights)
//! - Channel 015 (octal): Keyboard input from astronaut (key codes)
//!
//! This crate connects as a virtual DSKY, sends keystrokes, and reads
//! display state to verify AGC behavior.

pub mod channel_protocol;
pub mod dsky_client;
pub mod agc_runner;
pub mod dsky_display;
