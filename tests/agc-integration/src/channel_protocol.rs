//! yaAGC 4-byte channel I/O protocol.
//!
//! Each packet is 4 bytes encoding a channel number and 15-bit value:
//!
//! ```text
//! Byte 0: 00pppppp  (p = upper 6 bits of channel, bit 7 always 0)
//! Byte 1: 01pppddd  (p = lower 3 bits of channel, d = upper 3 bits of value)
//! Byte 2: 10dddddd  (d = middle 6 bits of value)
//! Byte 3: 11dddddd  (d = lower 6 bits of value)
//! ```
//!
//! The top 2 bits of each byte identify its position in the packet (00, 01, 10, 11).
//! This allows the receiver to resynchronize if bytes are lost.
//!
//! Reference: Virtual AGC documentation, yaAGC source code

/// A decoded channel I/O packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChannelPacket {
    /// Channel number (0-511, typically 0-177 octal)
    pub channel: u16,
    /// 15-bit value
    pub value: u16,
}

impl ChannelPacket {
    /// Create a new channel packet, masking inputs to valid widths.
    /// Channel is masked to 9 bits, value to 15 bits.
    pub fn new(channel: u16, value: u16) -> Self {
        ChannelPacket {
            channel: channel & 0x1FF,
            value: value & 0x7FFF,
        }
    }

    /// Encode this packet into 4 bytes for transmission to yaAGC.
    pub fn encode(&self) -> [u8; 4] {
        let ch = self.channel;
        let val = self.value;

        [
            // Byte 0: 00pppppp (upper 6 bits of channel)
            ((ch >> 3) & 0x3F) as u8,
            // Byte 1: 01pppddd (lower 3 bits of channel, upper 3 bits of value)
            0x40 | (((ch & 0x07) << 3) | ((val >> 12) & 0x07)) as u8,
            // Byte 2: 10dddddd (middle 6 bits of value)
            0x80 | ((val >> 6) & 0x3F) as u8,
            // Byte 3: 11dddddd (lower 6 bits of value)
            0xC0 | (val & 0x3F) as u8,
        ]
    }

    /// Decode a 4-byte packet received from yaAGC.
    ///
    /// Returns None if the byte markers (top 2 bits) don't match the expected
    /// pattern (00, 01, 10, 11), indicating a sync error.
    pub fn decode(bytes: &[u8; 4]) -> Option<Self> {
        // Verify byte position markers
        if (bytes[0] & 0xC0) != 0x00 {
            return None;
        }
        if (bytes[1] & 0xC0) != 0x40 {
            return None;
        }
        if (bytes[2] & 0xC0) != 0x80 {
            return None;
        }
        if (bytes[3] & 0xC0) != 0xC0 {
            return None;
        }

        let ch_hi = ((bytes[0] & 0x3F) as u16) << 3;
        let ch_lo = ((bytes[1] >> 3) & 0x07) as u16;
        let channel = ch_hi | ch_lo;

        let val_hi = ((bytes[1] & 0x07) as u16) << 12;
        let val_mid = ((bytes[2] & 0x3F) as u16) << 6;
        let val_lo = (bytes[3] & 0x3F) as u16;
        let value = val_hi | val_mid | val_lo;

        Some(ChannelPacket { channel, value })
    }
}

// Well-known AGC I/O channel numbers (octal)

/// DSKY keyboard input channel
pub const CHANNEL_KEYPRESSED: u16 = 0o15; // Channel 15 octal = 13 decimal
/// Actually, key presses go through channel 015 in the yaAGC protocol.
/// The AGC reads them via channel 015.

/// DSKY display relay word channels
pub const CHANNEL_DSALMOUT: u16 = 0o11;   // Channel 11: DSKY discrete alarm outputs
pub const CHANNEL_CHAN12: u16 = 0o12;       // Channel 12: DSKY discrete outputs
pub const CHANNEL_CHAN13: u16 = 0o13;       // Channel 13: DSKY output channel
pub const CHANNEL_CHAN10: u16 = 0o10;       // Channel 10: relay word output (display)

/// Input channels
pub const CHANNEL_CHAN15: u16 = 0o15;       // Channel 15: DSKY key code input
pub const CHANNEL_CHAN16: u16 = 0o16;       // Channel 16: navigation key input
pub const CHANNEL_CHAN32: u16 = 0o32;       // Channel 32: status flags

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let packet = ChannelPacket::new(0o15, 0o12345);
        let encoded = packet.encode();
        let decoded = ChannelPacket::decode(&encoded).unwrap();
        assert_eq!(decoded, packet);
    }

    #[test]
    fn test_encode_decode_zero() {
        let packet = ChannelPacket::new(0, 0);
        let encoded = packet.encode();
        let decoded = ChannelPacket::decode(&encoded).unwrap();
        assert_eq!(decoded, packet);
    }

    #[test]
    fn test_encode_decode_max() {
        let packet = ChannelPacket::new(0o777, 0o77777);
        let encoded = packet.encode();
        let decoded = ChannelPacket::decode(&encoded).unwrap();
        assert_eq!(decoded, packet);
    }

    #[test]
    fn test_byte_markers() {
        let packet = ChannelPacket::new(0o15, 0o12345);
        let encoded = packet.encode();
        assert_eq!(encoded[0] & 0xC0, 0x00, "byte 0 marker");
        assert_eq!(encoded[1] & 0xC0, 0x40, "byte 1 marker");
        assert_eq!(encoded[2] & 0xC0, 0x80, "byte 2 marker");
        assert_eq!(encoded[3] & 0xC0, 0xC0, "byte 3 marker");
    }

    #[test]
    fn test_decode_bad_marker_byte0() {
        let bad = [0xC0, 0x40, 0x80, 0xC0]; // byte 0 has 11 instead of 00
        assert!(ChannelPacket::decode(&bad).is_none());
    }

    #[test]
    fn test_decode_bad_marker_byte1() {
        let bad = [0x00, 0x00, 0x80, 0xC0]; // byte 1 has 00 instead of 01
        assert!(ChannelPacket::decode(&bad).is_none());
    }

    #[test]
    fn test_decode_bad_marker_byte2() {
        let bad = [0x00, 0x40, 0x40, 0xC0]; // byte 2 has 01 instead of 10
        assert!(ChannelPacket::decode(&bad).is_none());
    }

    #[test]
    fn test_decode_bad_marker_byte3() {
        let bad = [0x00, 0x40, 0x80, 0x80]; // byte 3 has 10 instead of 11
        assert!(ChannelPacket::decode(&bad).is_none());
    }

    #[test]
    fn test_golden_vector() {
        // Hand-computed encoding for channel 0o15 (13 decimal), value 0o12345 (5349 decimal)
        // Channel 0o15 = 0b0_001_101 (9 bits) → hi6 = 0b000001, lo3 = 0b101
        // Value 0o12345 = 0b1_010_011_100_101 (15 bits) → hi3=0b001, mid6=0b001110, lo6=0b000101
        //
        // Byte 0: 00|000001 = 0x01
        // Byte 1: 01|101_001 = 0x69
        // Byte 2: 10|001110 = 0x8E  (wait, let me recompute)
        //
        // value = 0o12345 = 1*4096 + 2*512 + 3*64 + 4*8 + 5 = 4096+1024+192+32+5 = 5349
        // 5349 in binary: 0001_0100_1110_0101
        // hi3 (bits 14-12): 001
        // mid6 (bits 11-6): 010011
        // lo6 (bits 5-0): 100101
        //
        // channel = 0o15 = 13 = 0b0001101
        // hi6 (bits 8-3): 000001
        // lo3 (bits 2-0): 101
        //
        // Byte 0: 00_000001 = 0x01
        // Byte 1: 01_101_001 = 0x69
        // Byte 2: 10_010011 = 0x93
        // Byte 3: 11_100101 = 0xE5
        let packet = ChannelPacket::new(0o15, 0o12345);
        let encoded = packet.encode();
        assert_eq!(encoded, [0x01, 0x69, 0x93, 0xE5], "golden vector mismatch");

        // Verify decode of the golden bytes produces the original packet
        let decoded = ChannelPacket::decode(&[0x01, 0x69, 0x93, 0xE5]).unwrap();
        assert_eq!(decoded.channel, 0o15);
        assert_eq!(decoded.value, 0o12345);
    }
}
