use std::fs::File;
use std::io::prelude::*;

use crate::compiler::*;
use crate::interpreter::*;
use crate::settings::*;
use crate::stdio_wrapper::*;

/// InterpreterBuilder
pub struct InterpreterBuilder {
    src: String,
    settings: Settings,
    reader: Box<dyn Read>,
    writer: Box<dyn Write>,
}

impl InterpreterBuilder {
    pub fn new(src: String, settings: Settings) -> Self {
        Self {
            src,
            settings,
            reader: Box::new(StdinWrapper::new()),
            writer: Box::new(StdoutWrapper::new()),
        }
    }

    /// Set input file. Default to stdin.
    pub fn reader<'a>(mut self, reader: Option<&'a str>) -> std::io::Result<Self> {
        match reader {
            Some(reader) => self.reader = Box::new(File::open(reader)?),
            None => self.reader = Box::new(std::io::stdin()),
        }

        Ok(self)
    }

    /// Set output file. Default to stdout.
    pub fn writer<'a>(mut self, writer: Option<&'a str>) -> std::io::Result<Self> {
        match writer {
            Some(writer) => self.writer = Box::new(File::open(writer)?),
            None => self.writer = Box::new(std::io::stdout()),
        }

        Ok(self)
    }

    pub fn build(self) -> Result<Interpreter, CompileError> {
        let program = Compiler::new().compile(self.src)?;

        Ok(Interpreter {
            program,
            settings: self.settings,
            reader: self.reader,
            writer: self.writer,
        })
    }
}
