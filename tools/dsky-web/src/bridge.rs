#![allow(dead_code)]
//! WebSocket bridge between yaAGC (TCP channel protocol) and browser.
//!
//! Translates yaAGC's 4-byte binary channel packets into JSON messages
//! for the DSKY web frontend, and converts browser keyboard events back
//! into AGC channel packets.
//!
//! JSON message format (AGC → browser):
//!   { "type": "channel", "channel": 10, "value": 12345 }
//!
//! JSON message format (browser → AGC):
//!   { "type": "key", "code": 17 }

/// Decoded relay word for the DSKY display.
#[derive(Clone, Debug, serde::Serialize)]
#[allow(dead_code)]
pub struct RelayWord {
    pub relay_num: u8,
    pub left_digit: u8,
    pub right_digit: u8,
    pub sign_bit: bool,
}

/// Parse a channel 010 relay word into its components.
#[allow(dead_code)]
pub fn parse_relay_word(value: u16) -> RelayWord {
    RelayWord {
        relay_num: ((value >> 11) & 0x0F) as u8,
        sign_bit: (value >> 10) & 1 == 1,
        left_digit: ((value >> 5) & 0x1F) as u8,
        right_digit: (value & 0x1F) as u8,
    }
}

/// 5-bit relay code to decimal digit lookup.
/// The codes are 7-segment bitmap patterns, not sequential.
pub fn relay_code_to_digit(code: u8) -> Option<u8> {
    match code {
        0o00 => None, // blank
        0o25 => Some(0),
        0o03 => Some(1),
        0o31 => Some(2),
        0o33 => Some(3),
        0o17 => Some(4),
        0o36 => Some(5),
        0o34 => Some(6),
        0o23 => Some(7),
        0o35 => Some(8),
        0o37 => Some(9),
        _ => None, // unknown pattern
    }
}

/// 5-bit relay code to 7-segment activation bitmap.
///
/// Returns which segments are on as a 7-bit mask:
/// ```text
///   bit 0: segment a (top)
///   bit 1: segment b (upper right)
///   bit 2: segment c (lower right)
///   bit 3: segment d (bottom)
///   bit 4: segment e (lower left)
///   bit 5: segment f (upper left)
///   bit 6: segment g (middle)
/// ```
pub fn relay_code_to_segments(code: u8) -> u8 {
    // Map the 5-bit relay code to 7-segment pattern.
    // The AGC's relay codes ARE the segment bitmaps, but in a different
    // bit ordering than our standard a-g convention.
    //
    // AGC relay bit mapping (from PINBALL_GAME_BUTTONS_AND_LIGHTS.agc):
    //   bit 0: segment b (upper right)
    //   bit 1: segment c (lower right)
    //   bit 2: segment a (top)
    //   bit 3: segment d (bottom) + segment e (lower left)
    //   bit 4: segment f (upper left) + segment g (middle)
    //
    // We remap to standard a(0)-g(6) for the frontend.
    match code {
        0o00 => 0b0000000, // blank
        0o25 => 0b0111111, // 0: a,b,c,d,e,f
        0o03 => 0b0000110, // 1: b,c
        0o31 => 0b1011011, // 2: a,b,d,e,g
        0o33 => 0b1001111, // 3: a,b,c,d,g
        0o17 => 0b1100110, // 4: b,c,f,g
        0o36 => 0b1101101, // 5: a,c,d,f,g
        0o34 => 0b1111101, // 6: a,c,d,e,f,g
        0o23 => 0b0000111, // 7: a,b,c
        0o35 => 0b1111111, // 8: a,b,c,d,e,f,g
        0o37 => 0b1101111, // 9: a,b,c,d,f,g
        _ => 0b0000000,
    }
}

/// DSKY indicator light state decoded from channel 011.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DskyLights {
    pub comp_acty: bool,
    pub uplink_acty: bool,
    pub temp: bool,
    pub key_rel: bool,
    pub verb_noun_flash: bool,
    pub oper_err: bool,
    pub restart: bool,
    pub stby: bool,
    pub prog_alarm: bool,
    pub tracker: bool,
    pub alt: bool,
    pub vel: bool,
    pub no_att: bool,
    pub gimbal_lock: bool,
    pub prog: bool,
}

impl DskyLights {
    /// Update from channel 011 (DSALMOUT) value.
    pub fn update_channel_11(&mut self, value: u16) {
        self.oper_err = (value >> 6) & 1 == 1;
        self.verb_noun_flash = (value >> 5) & 1 == 1;
        self.key_rel = (value >> 4) & 1 == 1;
    }

    /// Update from channel 013 value.
    pub fn update_channel_13(&mut self, value: u16) {
        self.restart = (value >> 0) & 1 == 1;
        self.stby = (value >> 1) & 1 == 1;
        self.prog_alarm = (value >> 2) & 1 == 1;
        self.gimbal_lock = (value >> 3) & 1 == 1;
        self.tracker = (value >> 4) & 1 == 1;
        self.alt = (value >> 5) & 1 == 1;
        self.vel = (value >> 6) & 1 == 1;
        self.no_att = (value >> 7) & 1 == 1;
        self.prog = (value >> 8) & 1 == 1;
        self.temp = (value >> 9) & 1 == 1;
        self.uplink_acty = (value >> 10) & 1 == 1;
        self.comp_acty = (value >> 11) & 1 == 1;
    }
}

/// Full DSKY display state sent to the browser as JSON.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DskyState {
    /// 7-segment data for each display position.
    /// Index 0-1: PROG (MD1, MD2)
    /// Index 2-3: VERB (VD1, VD2)
    /// Index 4-5: NOUN (ND1, ND2)
    /// Index 6-10: R1 (5 digits)
    /// Index 11-15: R2 (5 digits)
    /// Index 16-20: R3 (5 digits)
    pub segments: [u8; 21],
    /// Sign indicators: +R1, -R1, +R2, -R2, +R3, -R3
    pub signs: [bool; 6],
    /// Indicator lights.
    pub lights: DskyLights,
}
