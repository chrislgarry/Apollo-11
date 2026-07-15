//! DSKY WebSocket Bridge
//!
//! Connects to yaAGC via TCP (channel protocol) and exposes a WebSocket
//! server for the DSKY web frontend.
//!
//! Usage:
//!   dsky-bridge [--agc-host HOST:PORT] [--ws-port PORT]
//!
//! Defaults:
//!   --agc-host localhost:19697  (yaAGC default port)
//!   --ws-port 8765              (WebSocket for browser)

mod bridge;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tungstenite::accept;
use tungstenite::Message;

/// 4-byte yaAGC channel packet encode/decode.
/// (Duplicated from agc-integration to keep this binary standalone.)
fn encode_packet(channel: u16, value: u16) -> [u8; 4] {
    [
        ((channel >> 3) & 0x3F) as u8,
        0x40 | (((channel & 0x07) << 3) | ((value >> 12) & 0x07)) as u8,
        0x80 | ((value >> 6) & 0x3F) as u8,
        0xC0 | (value & 0x3F) as u8,
    ]
}

fn decode_packet(bytes: &[u8; 4]) -> Option<(u16, u16)> {
    if (bytes[0] & 0xC0) != 0x00 { return None; }
    if (bytes[1] & 0xC0) != 0x40 { return None; }
    if (bytes[2] & 0xC0) != 0x80 { return None; }
    if (bytes[3] & 0xC0) != 0xC0 { return None; }
    let ch = (((bytes[0] & 0x3F) as u16) << 3) | ((bytes[1] >> 3) & 0x07) as u16;
    let val = (((bytes[1] & 0x07) as u16) << 12)
        | (((bytes[2] & 0x3F) as u16) << 6)
        | (bytes[3] & 0x3F) as u16;
    Some((ch, val))
}

/// Messages queued for the WebSocket client.
type WsQueue = Arc<Mutex<Vec<String>>>;

/// Read from yaAGC TCP stream and queue JSON messages for WebSocket.
fn agc_reader(mut agc_stream: TcpStream, queue: WsQueue) {
    let mut buf = [0u8; 4];
    loop {
        match agc_stream.read_exact(&mut buf) {
            Ok(()) => {
                if let Some((channel, value)) = decode_packet(&buf) {
                    let json = format!(
                        r#"{{"type":"channel","channel":{},"value":{}}}"#,
                        channel, value
                    );
                    if let Ok(mut q) = queue.lock() {
                        q.push(json);
                        // Prevent unbounded growth if WS client is slow
                        if q.len() > 1000 {
                            q.drain(0..500);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("[agc-reader] Connection lost: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let agc_host = std::env::args()
        .skip_while(|a| a != "--agc-host")
        .nth(1)
        .unwrap_or_else(|| "localhost:19697".to_string());

    let ws_port: u16 = std::env::args()
        .skip_while(|a| a != "--ws-port")
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(8765);

    eprintln!("[dsky-bridge] Connecting to yaAGC at {}...", agc_host);
    let agc_stream = match TcpStream::connect(&agc_host) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[dsky-bridge] Failed to connect to yaAGC: {}", e);
            eprintln!("[dsky-bridge] Make sure yaAGC is running.");
            std::process::exit(1);
        }
    };
    eprintln!("[dsky-bridge] Connected to yaAGC");

    let agc_write = agc_stream.try_clone().expect("Failed to clone AGC stream");
    let agc_write = Arc::new(Mutex::new(agc_write));

    // Queue for AGC → browser messages
    let queue: WsQueue = Arc::new(Mutex::new(Vec::new()));

    // Start AGC reader thread
    let reader_queue = queue.clone();
    thread::spawn(move || agc_reader(agc_stream, reader_queue));

    // Start WebSocket server
    let ws_addr = format!("0.0.0.0:{}", ws_port);
    let listener = TcpListener::bind(&ws_addr).expect("Failed to bind WebSocket port");
    eprintln!(
        "[dsky-bridge] WebSocket server on ws://localhost:{}",
        ws_port
    );
    eprintln!("[dsky-bridge] Open tools/dsky-web/public/index.html in a browser");

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[dsky-bridge] Accept error: {}", e);
                continue;
            }
        };

        let ws_queue = queue.clone();
        let ws_agc_write = agc_write.clone();

        thread::spawn(move || {
            let mut websocket = match accept(stream) {
                Ok(ws) => ws,
                Err(e) => {
                    eprintln!("[dsky-bridge] WebSocket handshake failed: {}", e);
                    return;
                }
            };
            eprintln!("[dsky-bridge] Browser connected");

            // Set non-blocking for polling pattern
            websocket
                .get_ref()
                .set_nonblocking(true)
                .ok();

            loop {
                // Send queued AGC messages to browser
                if let Ok(mut q) = ws_queue.lock() {
                    for json in q.drain(..) {
                        if websocket.send(Message::Text(json)).is_err() {
                            eprintln!("[dsky-bridge] Browser disconnected (send)");
                            return;
                        }
                    }
                }

                // Read browser messages (keyboard)
                match websocket.read() {
                    Ok(Message::Text(text)) => {
                        if let Ok(msg) = serde_json::from_str::<serde_json::Value>(&text) {
                            if msg["type"] == "key" {
                                if let Some(code) = msg["code"].as_u64() {
                                    let packet = encode_packet(0o15, code as u16);
                                    if let Ok(mut writer) = ws_agc_write.lock() {
                                        let _ = writer.write_all(&packet);
                                    }
                                    // Send key release after small delay
                                    thread::sleep(Duration::from_millis(30));
                                    let release = encode_packet(0o15, 0);
                                    if let Ok(mut writer) = ws_agc_write.lock() {
                                        let _ = writer.write_all(&release);
                                    }
                                }
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        eprintln!("[dsky-bridge] Browser disconnected");
                        return;
                    }
                    Err(tungstenite::Error::Io(ref e))
                        if e.kind() == std::io::ErrorKind::WouldBlock =>
                    {
                        // No message available, continue polling
                    }
                    Err(_) => {
                        // Ignore other errors, continue
                    }
                    _ => {}
                }

                thread::sleep(Duration::from_millis(16)); // ~60fps
            }
        });
    }
}
