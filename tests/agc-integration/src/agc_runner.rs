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

        let mut child = Command::new(yaagc_path)
            .arg(format!("--port={}", port))
            .arg(bin_path)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        // Check for immediate exit (bad args, missing binary, port conflict)
        thread::sleep(Duration::from_millis(50));
        if let Ok(Some(status)) = child.try_wait() {
            let stderr = Self::drain_stderr(&mut child);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("yaAGC exited immediately with {}: {}", status, stderr),
            ));
        }

        // Wait for port readiness
        thread::sleep(Duration::from_millis(450));

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

    /// Stop the yaAGC process. Tolerates the process having already exited.
    pub fn stop(&mut self) -> io::Result<()> {
        match self.child.kill() {
            Ok(()) => {}
            Err(ref e) if e.kind() == io::ErrorKind::InvalidInput => {
                // Process already exited — not an error
            }
            Err(e) => return Err(e),
        }
        self.child.wait()?;
        Ok(())
    }

    /// Check if yaAGC is still running.
    pub fn is_running(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }

    /// Read any available stderr output from yaAGC (for crash diagnostics).
    pub fn captured_stderr(&mut self) -> String {
        Self::drain_stderr(&mut self.child)
    }

    /// Drain stderr from a child process without blocking.
    fn drain_stderr(child: &mut Child) -> String {
        use std::io::Read;
        let Some(stderr) = child.stderr.as_mut() else {
            return String::new();
        };
        let mut buf = String::new();
        // Don't block — just read what's available
        let _ = stderr.read_to_string(&mut buf);
        buf
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
    match Command::new("yaAGC").arg("--help").output() {
        Ok(output) if output.status.success() => return Some("yaAGC".to_string()),
        Ok(output) => eprintln!("yaAGC found but exited with {}", output.status),
        Err(ref e) if e.kind() != io::ErrorKind::NotFound => {
            eprintln!("yaAGC found but failed to run: {}", e);
        }
        Err(_) => {} // genuinely not found on PATH
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
