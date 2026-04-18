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
    /// Create a new channel packet.
    pub fn new(channel: u16, value: u16) -> Self {
        assert!(channel < 512, "channel {} exceeds 9 bits", channel);
        assert!(value < 32768, "value {} exceeds 15 bits", value);
        ChannelPacket { channel, value }
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
    fn test_decode_bad_markers() {
        let bad = [0xFF, 0x40, 0x80, 0xC0]; // byte 0 marker wrong
        assert!(ChannelPacket::decode(&bad).is_none());
    }
}
