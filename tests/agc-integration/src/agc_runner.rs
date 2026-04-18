//! yaAGC process management.
//!
//! Starts and stops yaAGC simulator instances, managing the subprocess
//! lifecycle and providing the TCP port for DSKY client connections.

use std::io;
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

/// Default yaAGC listen port for peripheral connections.
pub const DEFAULT_AGC_PORT: u16 = 19697;

/// A running yaAGC simulator instance.
pub struct AgcRunner {
    child: Child,
    port: u16,
}

impl AgcRunner {
    /// Start a yaAGC instance with the given binary file.
    ///
    /// - `yaagc_path`: Path to the yaAGC executable
    /// - `bin_path`: Path to the assembled .bin rope image
    /// - `port`: TCP port for peripheral connections (default: 19697)
    ///
    /// The yaAGC process runs in the background. Connect to it via
    /// `DskyClient::connect("localhost:{port}")`.
    pub fn start(yaagc_path: &str, bin_path: &str, port: Option<u16>) -> io::Result<Self> {
        let port = port.unwrap_or(DEFAULT_AGC_PORT);

        let child = Command::new(yaagc_path)
            .arg(format!("--port={}", port))
            .arg(bin_path)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()?;

        // Give yaAGC time to start and begin listening
        thread::sleep(Duration::from_millis(500));

        Ok(AgcRunner { child, port })
    }

    /// The TCP port this yaAGC instance is listening on.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// The connection address for DSKY clients.
    pub fn address(&self) -> String {
        format!("localhost:{}", self.port)
    }

    /// Stop the yaAGC process.
    pub fn stop(&mut self) -> io::Result<()> {
        self.child.kill()?;
        self.child.wait()?;
        Ok(())
    }

    /// Check if yaAGC is still running.
    pub fn is_running(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }
}

impl Drop for AgcRunner {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Find yaAGC on PATH or in common locations.
pub fn find_yaagc() -> Option<String> {
    // Check PATH first
    if Command::new("yaAGC").arg("--help").output().is_ok() {
        return Some("yaAGC".to_string());
    }

    // Check common install locations
    let paths = [
        "/usr/local/bin/yaAGC",
        "/usr/bin/yaAGC",
        "../../tools/yaAGC", // relative to tests/ directory
    ];

    for path in &paths {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }

    None
}
