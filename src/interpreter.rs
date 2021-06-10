use std::io::prelude::*;

use crate::cells::*;
use crate::error::*;
use crate::program::*;
use crate::settings::*;

/// State when running
pub struct State {
    pub cells: Cells,
    pub command_ptr: usize,
    pub cell_ptr: usize,
}

/// Read result
enum ReadResult {
    Success(u8),
    None,
    Newline,
    ReadFailed,
    ParseNumError,
}

/// Interpreter
pub struct Interpreter {
    pub program: Program,
    pub settings: Settings,
    pub reader: Box<dyn Read>,
    pub writer: Box<dyn Write>,
}

impl Interpreter {
    /// Return a new state for running.
    pub fn ready(&self) -> State {
        State {
            cells: Cells::new(self.settings.array_size),
            command_ptr: 0,
            cell_ptr: 0,
        }
    }

    /// Execute current instruction
    pub fn next(&mut self, state: &mut State) -> RunResult {
        match self.program.0[state.command_ptr] {
            Command::Halt => return RunResult::Halted,
            Command::Right => {
                if state.cell_ptr == state.cells.0.len() - 1 && !self.settings.dynamic_size {
                    state.cell_ptr = 0;
                } else {
                    state.cell_ptr = state.cell_ptr.wrapping_add(1);
                }
            }
            Command::Left => {
                if state.cell_ptr == 0 && self.settings.wrapping {
                    state.cell_ptr = state.cells.0.len() - 1;
                } else {
                    state.cell_ptr = state.cell_ptr.wrapping_sub(1);
                }
            }
            Command::Inc => match self.get_cell(state) {
                Some(data) => *data = data.wrapping_add(1),
                None => return RunResult::IndexOutOfBound(state.cell_ptr),
            },
            Command::Dec => match self.get_cell(state) {
                Some(data) => *data = data.wrapping_sub(1),
                None => return RunResult::IndexOutOfBound(state.cell_ptr),
            },
            Command::Open(dst) => match self.get_cell(state) {
                Some(data) => {
                    if *data == 0 {
                        state.command_ptr = dst;
                    }
                }
                None => return RunResult::IndexOutOfBound(state.cell_ptr),
            },
            Command::Close(dst) => match self.get_cell(state) {
                Some(data) => {
                    if *data != 0 {
                        state.command_ptr = dst;
                    }
                }
                None => return RunResult::IndexOutOfBound(state.cell_ptr),
            },
            Command::Read => {
                let result = match self.settings.input_mode {
                    InputMode::Ascii => self.read_ascii(),
                    InputMode::Digit => self.read_digit(),
                };

                match result {
                    ReadResult::Success(val) => match self.get_cell(state) {
                        Some(data) => *data = val,
                        None => return RunResult::IndexOutOfBound(state.cell_ptr),
                    },
                    ReadResult::None => match self.settings.eof_behavior {
                        EofBehavior::AsIs => {}
                        EofBehavior::NegativeOne => match self.get_cell(state) {
                            Some(data) => *data = u8::MAX,
                            None => return RunResult::IndexOutOfBound(state.cell_ptr),
                        },
                        EofBehavior::Zero => match self.get_cell(state) {
                            Some(data) => *data = 0,
                            None => return RunResult::IndexOutOfBound(state.cell_ptr),
                        },
                    },
                    ReadResult::Newline => match self.get_cell(state) {
                        Some(data) => *data = '\n' as u8,
                        None => return RunResult::IndexOutOfBound(state.cell_ptr),
                    },
                    ReadResult::ReadFailed => return RunResult::ReadFailed,
                    ReadResult::ParseNumError => return RunResult::ParseNumError,
                }
            }
            Command::Write => {
                let buf = match self.get_cell(state) {
                    Some(data) => {
                        if *data == '\n' as u8
                            && matches!(self.settings.newline_mode, NewlineMode::CRLF)
                        {
                            ['\r' as u8, '\n' as u8]
                        } else {
                            [*data, 0]
                        }
                    }
                    None => return RunResult::IndexOutOfBound(state.cell_ptr),
                };

                match self.writer.write(&buf) {
                    Err(_) => return RunResult::WriteFailed,
                    Ok(0) => return RunResult::WriteFailed,
                    Ok(_) => match self.writer.flush() {
                        Err(_) => return RunResult::WriteFailed,
                        Ok(_) => {}
                    },
                }
            }
        }
        state.command_ptr += 1;
        RunResult::None
    }

    fn read_ascii(&mut self) -> ReadResult {
        return match self.settings.newline_mode {
            NewlineMode::CRLF => self.read_ascii_crlf(),
            NewlineMode::LF => self.read_ascii_lf(),
        };
    }

    fn read_ascii_crlf(&mut self) -> ReadResult {
        let mut buf = [0];

        match self.reader.read(&mut buf) {
            Err(_) => return ReadResult::ReadFailed,
            Ok(0) => return ReadResult::None,
            Ok(_) => {
                if buf[0] != '\r' as u8 {
                    return ReadResult::Success(buf[0]);
                }
            }
        }

        match self.reader.read(&mut buf) {
            Err(_) => return ReadResult::ReadFailed,
            Ok(0) => return ReadResult::Success('\r' as u8),
            Ok(_) => {
                if buf[0] != '\n' as u8 {
                    return ReadResult::Success(buf[0]);
                } else if !self.settings.ignore_newline {
                    return ReadResult::Newline;
                }
            }
        }

        match self.reader.read(&mut buf) {
            Err(_) => return ReadResult::ReadFailed,
            Ok(0) => return ReadResult::ReadFailed,
            Ok(_) => return ReadResult::Success(buf[0]),
        }
    }

    fn read_ascii_lf(&mut self) -> ReadResult {
        let mut buf = [0];

        match self.reader.read(&mut buf) {
            Err(_) => return ReadResult::ReadFailed,
            Ok(0) => return ReadResult::None,
            Ok(_) => {
                if buf[0] != '\n' as u8 {
                    return ReadResult::Success(buf[0]);
                } else if !self.settings.ignore_newline {
                    return ReadResult::Newline;
                }
            }
        }

        match self.reader.read(&mut buf) {
            Err(_) => return ReadResult::ReadFailed,
            Ok(0) => return ReadResult::ReadFailed,
            Ok(_) => {
                return ReadResult::Success(buf[0]);
            }
        }
    }

    fn read_digit(&mut self) -> ReadResult {
        let mut result = 0;
        let mut last_is_cr = false;
        loop {
            let mut buf = [0];
            match self.reader.read(&mut buf) {
                Err(_) => return ReadResult::ReadFailed,
                Ok(0) => return ReadResult::None,
                Ok(_) => match self.settings.newline_mode {
                    NewlineMode::CRLF => {
                        if buf[0] == '\r' as u8 && !last_is_cr {
                            // read '\r' first time
                            last_is_cr = true;
                        } else if buf[0] == '\n' as u8 && last_is_cr {
                            // read '\n' right after '\r' (i.e. newline)
                            return ReadResult::Success(result);
                        } else if buf[0].is_ascii_digit() && !last_is_cr {
                            // read a digit
                            result = result * 10 + buf[0] - '0' as u8;
                            last_is_cr = false;
                        } else {
                            // read something else
                            return ReadResult::ParseNumError;
                        }
                    }
                    NewlineMode::LF => {
                        if buf[0] == '\n' as u8 {
                            return ReadResult::Success(result);
                        } else if buf[0].is_ascii_digit() {
                            result = result * 10 + buf[0] - '0' as u8;
                        } else {
                            return ReadResult::ParseNumError;
                        }
                    }
                },
            }
        }
    }

    fn get_cell<'a>(&mut self, state: &'a mut State) -> Option<&'a mut u8> {
        if self.settings.dynamic_size {
            return state.cells.get_mut_expand(state.cell_ptr);
        } else {
            return state.cells.get_mut(state.cell_ptr);
        }
    }
}
