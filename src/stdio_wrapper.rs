//! Since `Stdin` does not lock on `Read::read`, we have to create a wrapper of stdin
//! that automatically lock on read. While at it, we also create a wrapper of stdout
//! for conformity.

use std::io::prelude::*;

/// Wrapper around Stdin that automatically lock on read.
pub struct StdinWrapper {
    stdin: std::io::Stdin,
}

impl StdinWrapper {
    pub fn new() -> Self {
        Self {
            stdin: std::io::stdin(),
        }
    }
}

impl Read for StdinWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stdin.lock().read(buf)
    }
}

/// Wrapper arround Stdout that automatically lock on write.
pub struct StdoutWrapper {
    stdout: std::io::Stdout,
}

impl StdoutWrapper {
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
        }
    }
}

impl Write for StdoutWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.lock().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}
