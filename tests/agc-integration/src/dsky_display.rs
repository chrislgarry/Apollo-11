//! DSKY display state parser.
//!
//! The AGC sends display data to the DSKY via channel 010 (relay word output).
//! Each relay word encodes which display register to update and the digit data.
//!
//! The DSKY has the following displays:
//! - PROG: 2-digit program number (R1 position)
//! - VERB: 2-digit verb number
//! - NOUN: 2-digit noun number
//! - R1, R2, R3: 5-digit signed registers with +/- sign
//!
//! Channel 010 relay word format (15 bits):
//!   Bits 15-12: relay word number (selects which display register)
//!   Bits 11-1: data (digit codes, typically 5-bit per digit)
//!
//! Channel 011 (DSALMOUT): discrete alarm/status bits
//! Channel 013: discrete output bits (lights)
//!
//! Reference: Comanche055/PINBALL_GAME_BUTTONS_AND_LIGHTS.agc

/// The current DSKY display state.
#[derive(Clone, Debug, Default)]
pub struct DskyDisplay {
    /// Program number (0-99)
    pub prog: u8,
    /// Verb number (0-99)
    pub verb: u8,
    /// Noun number (0-99)
    pub noun: u8,
    /// Register 1 value (displayed as 5 digits with sign)
    pub r1: i32,
    /// Register 2 value
    pub r2: i32,
    /// Register 3 value
    pub r3: i32,
    /// Raw relay words received (for debugging)
    pub relay_words: Vec<(u8, u16)>,
    /// PROG ALARM light on
    pub prog_alarm: bool,
    /// Various warning lights from channel 011
    pub lights: u16,
}

/// Relay word numbers that select DSKY display positions.
/// These are the top 4 bits of the channel 010 output.
pub mod relay {
    pub const MD1: u8 = 11;  // PROG digit 1 (tens)
    pub const MD2: u8 = 10;  // PROG digit 2 (ones)
    pub const VD1: u8 = 9;   // VERB digit 1
    pub const VD2: u8 = 8;   // VERB digit 2
    pub const ND1: u8 = 7;   // NOUN digit 1
    pub const ND2: u8 = 6;   // NOUN digit 2
    pub const R1D1: u8 = 5;  // R1 sign and digit 1
    pub const R1D2: u8 = 4;  // R1 digits 2-3
    pub const R1D3: u8 = 3;  // R1 digits 4-5
    pub const R2D1: u8 = 2;  // R2 sign and digit 1
    pub const R2D2: u8 = 1;  // R2 digits 2-3
    pub const R2D3: u8 = 12; // R2 digits 4-5 (wraps)
    pub const R3D1: u8 = 13; // R3 sign and digit 1
    pub const R3D2: u8 = 14; // R3 digits 2-3
    pub const R3D3: u8 = 15; // R3 digits 4-5
}

/// Decode a 5-bit digit code from the relay word.
/// The DSKY uses a 5-bit encoding for each displayed digit:
///   0 = blank, 1-10 = digits 0-9 (offset by 1)
pub fn decode_digit(code: u8) -> Option<u8> {
    match code & 0x1F {
        0 => None,     // blank
        n @ 1..=10 => Some(n - 1),
        _ => None,     // invalid
    }
}

impl DskyDisplay {
    pub fn new() -> Self {
        Self::default()
    }

    /// Process a relay word from channel 010.
    pub fn update_relay_word(&mut self, relay_num: u8, data: u16) {
        self.relay_words.push((relay_num, data));
        // The exact bit layout of relay words varies by position.
        // For now, store raw data for inspection.
    }

    /// Process a channel 011 output (discrete alarm lights).
    pub fn update_channel_11(&mut self, value: u16) {
        self.lights = value;
        // Bit 1 of channel 011 is typically the PROG ALARM indicator
        self.prog_alarm = (value & 0x02) != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- decode_digit tests ---

    #[test]
    fn test_decode_digit_blank() {
        assert_eq!(decode_digit(0), None);
    }

    #[test]
    fn test_decode_digit_zero() {
        // Code 1 → digit 0 (offset by 1)
        assert_eq!(decode_digit(1), Some(0));
    }

    #[test]
    fn test_decode_digit_nine() {
        // Code 10 → digit 9
        assert_eq!(decode_digit(10), Some(9));
    }

    #[test]
    fn test_decode_digit_all_valid() {
        for code in 1..=10u8 {
            assert_eq!(decode_digit(code), Some(code - 1));
        }
    }

    #[test]
    fn test_decode_digit_invalid() {
        // Codes 11-31 are invalid
        for code in 11..=31u8 {
            assert_eq!(decode_digit(code), None, "code {} should be invalid", code);
        }
    }

    #[test]
    fn test_decode_digit_masks_to_5_bits() {
        // High bits should be masked off: 0x20 | 1 = 33, but & 0x1F = 1 → digit 0
        assert_eq!(decode_digit(0x21), Some(0));
    }

    // --- update_channel_11 tests ---

    #[test]
    fn test_prog_alarm_bit_on() {
        let mut d = DskyDisplay::new();
        d.update_channel_11(0x02); // bit 1 set
        assert!(d.prog_alarm, "PROG ALARM should be on when bit 1 set");
    }

    #[test]
    fn test_prog_alarm_bit_off() {
        let mut d = DskyDisplay::new();
        d.update_channel_11(0x00);
        assert!(!d.prog_alarm, "PROG ALARM should be off when no bits set");
    }

    #[test]
    fn test_prog_alarm_adjacent_bit_not_alarm() {
        let mut d = DskyDisplay::new();
        d.update_channel_11(0x01); // bit 0, not bit 1
        assert!(!d.prog_alarm, "bit 0 should not trigger PROG ALARM");
    }

    #[test]
    fn test_prog_alarm_all_bits() {
        let mut d = DskyDisplay::new();
        d.update_channel_11(0xFFFF); // all bits set
        assert!(d.prog_alarm, "PROG ALARM should be on with all bits set");
    }

    #[test]
    fn test_lights_stored() {
        let mut d = DskyDisplay::new();
        d.update_channel_11(0o1234);
        assert_eq!(d.lights, 0o1234);
    }

    // --- relay word tests ---

    #[test]
    fn test_relay_word_stored() {
        let mut d = DskyDisplay::new();
        d.update_relay_word(11, 0x1FF);
        assert_eq!(d.relay_words.len(), 1);
        assert_eq!(d.relay_words[0], (11, 0x1FF));
    }
}
