//! AGC arithmetic model for the Apollo Guidance Computer (Block II).
//!
//! This crate provides a faithful software model of the AGC's 15-bit
//! 1's complement arithmetic, including the interpreter's pseudo-instructions
//! for double-precision math, trigonometry, and vector operations.
//!
//! The types and operations here are derived directly from the AGC source code
//! in the Comanche055 (Command Module) and Luminary099 (Lunar Module) programs.

pub mod ones_complement;
pub mod constants;
pub mod arithmetic;
pub mod poly;
pub mod trig;
pub mod sqrt;
pub mod vector;
