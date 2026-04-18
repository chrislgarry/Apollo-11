//! Virtual DSKY client that connects to yaAGC over TCP.
//!
//! This module provides a high-level interface for interacting with the AGC
//! as if you were an astronaut pressing buttons on the DSKY:
//!
//! - Press individual keys (VERB, NOUN, ENTER, CLEAR, 0-9, +, -)
//! - Send verb/noun sequences (e.g., V16N36E for "display time")
//! - Read display state (PROG, VERB, NOUN, R1, R2, R3)
//! - Wait for specific display conditions
//!
//! ## DSKY Key Codes (Channel 015)
//!
//! The DSKY keyboard sends 5-bit key codes via channel 015:
//! ```text
//! VERB  = 17 (octal 21)
//! NOUN  = 31 (octal 37)
//! ENTER = 28 (octal 34)
//! CLEAR = 30 (octal 36)
//! KEY 0 = 16 (octal 20)
//! KEY 1 = 1
//! KEY 2 = 2
//! ...
//! KEY 9 = 9
//! KEY + = 26 (octal 32)
//! KEY - = 27 (octal 33)
//! ```

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use crate::channel_protocol::{ChannelPacket, CHANNEL_CHAN15, CHANNEL_CHAN10, CHANNEL_DSALMOUT};
use crate::dsky_display::DskyDisplay;

/// DSKY key codes sent via channel 015.
pub mod key {
    pub const ZERO: u16 = 16;
    pub const ONE: u16 = 1;
    pub const TWO: u16 = 2;
    pub const THREE: u16 = 3;
    pub const FOUR: u16 = 4;
    pub const FIVE: u16 = 5;
    pub const SIX: u16 = 6;
    pub const SEVEN: u16 = 7;
    pub const EIGHT: u16 = 8;
    pub const NINE: u16 = 9;
    pub const VERB: u16 = 17;
    pub const NOUN: u16 = 31;
    pub const ENTER: u16 = 28;
    pub const CLEAR: u16 = 30;
    pub const PLUS: u16 = 26;
    pub const MINUS: u16 = 27;
    pub const KEY_RELEASE: u16 = 0;
}

/// Convert a digit character to a DSKY key code.
fn digit_to_key(ch: char) -> Option<u16> {
    match ch {
        '0' => Some(key::ZERO),
        '1' => Some(key::ONE),
        '2' => Some(key::TWO),
        '3' => Some(key::THREE),
        '4' => Some(key::FOUR),
        '5' => Some(key::FIVE),
        '6' => Some(key::SIX),
        '7' => Some(key::SEVEN),
        '8' => Some(key::EIGHT),
        '9' => Some(key::NINE),
        _ => None,
    }
}

/// A virtual DSKY client connected to a yaAGC instance.
pub struct DskyClient {
    stream: TcpStream,
    display: DskyDisplay,
}

impl DskyClient {
    /// Connect to a yaAGC instance at the given address (e.g., "localhost:19697").
    pub fn connect(addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(false)?;
        stream.set_read_timeout(Some(Duration::from_millis(100)))?;

        Ok(DskyClient {
            stream,
            display: DskyDisplay::new(),
        })
    }

    /// Send a key press to the DSKY.
    pub fn press_key(&mut self, key_code: u16) -> io::Result<()> {
        let packet = ChannelPacket::new(CHANNEL_CHAN15, key_code);
        self.stream.write_all(&packet.encode())?;

        // Small delay to simulate key press duration
        std::thread::sleep(Duration::from_millis(50));

        // Send key release
        let release = ChannelPacket::new(CHANNEL_CHAN15, key::KEY_RELEASE);
        self.stream.write_all(&release.encode())?;

        // Process any incoming display updates
        self.poll_display(Duration::from_millis(100))?;

        Ok(())
    }

    /// Send a verb-noun-enter sequence: V{vv}N{nn}E
    pub fn verb_noun(&mut self, verb: u8, noun: u8) -> io::Result<()> {
        // VERB key
        self.press_key(key::VERB)?;

        // Verb digits
        let v_tens = verb / 10;
        let v_ones = verb % 10;
        if let Some(k) = digit_to_key(char::from(b'0' + v_tens)) {
            self.press_key(k)?;
        }
        if let Some(k) = digit_to_key(char::from(b'0' + v_ones)) {
            self.press_key(k)?;
        }

        // NOUN key
        self.press_key(key::NOUN)?;

        // Noun digits
        let n_tens = noun / 10;
        let n_ones = noun % 10;
        if let Some(k) = digit_to_key(char::from(b'0' + n_tens)) {
            self.press_key(k)?;
        }
        if let Some(k) = digit_to_key(char::from(b'0' + n_ones)) {
            self.press_key(k)?;
        }

        // ENTER
        self.press_key(key::ENTER)?;

        Ok(())
    }

    /// Read and process incoming display updates for the given duration.
    pub fn poll_display(&mut self, duration: Duration) -> io::Result<()> {
        let start = Instant::now();
        let mut buf = [0u8; 4];

        while start.elapsed() < duration {
            match self.stream.read_exact(&mut buf) {
                Ok(()) => {
                    if let Some(packet) = ChannelPacket::decode(&buf) {
                        self.process_packet(packet);
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock
                    || e.kind() == io::ErrorKind::TimedOut =>
                {
                    break;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    /// Wait until a predicate on the display state is satisfied, or timeout.
    pub fn wait_for_display<F>(
        &mut self,
        predicate: F,
        timeout: Duration,
    ) -> io::Result<bool>
    where
        F: Fn(&DskyDisplay) -> bool,
    {
        let start = Instant::now();
        while start.elapsed() < timeout {
            self.poll_display(Duration::from_millis(100))?;
            if predicate(&self.display) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Get a snapshot of the current display state.
    pub fn display(&self) -> &DskyDisplay {
        &self.display
    }

    /// Process an incoming packet and update display state.
    fn process_packet(&mut self, packet: ChannelPacket) {
        match packet.channel {
            CHANNEL_CHAN10 => {
                // Relay word: top 4 bits = relay number, rest = data
                let relay_num = ((packet.value >> 11) & 0x0F) as u8;
                let data = packet.value & 0x07FF;
                self.display.update_relay_word(relay_num, data);
            }
            CHANNEL_DSALMOUT => {
                self.display.update_channel_11(packet.value);
            }
            _ => {
                // Other channels (e.g., channel 12, 13) can be processed later
            }
        }
    }
}
