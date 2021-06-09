use crate::program::*;

/// Error during compilation
#[derive(Debug)]
pub enum CompileError {
    /// Syntactic error
    ///
    /// - `usize` is the location of the error
    /// - `char` is the unknown character
    Syntax(usize, char),
    /// Not all brackets have a matching bracket
    ///
    /// - `usize` is the location of the error
    UnmatchedBracket(usize),
}

/// Compiler
pub struct Compiler {
    commands: Vec<Command>,
    opens: Vec<usize>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            commands: Vec::new(),
            opens: Vec::new(),
        }
    }

    pub fn compile(mut self, src: String) -> Result<Program, CompileError> {
        for (i, ch) in src.chars().enumerate() {
            match ch {
                '>' => self.commands.push(Command::Right),
                '<' => self.commands.push(Command::Left),
                '+' => self.commands.push(Command::Inc),
                '-' => self.commands.push(Command::Dec),
                '[' => {
                    self.opens.push(self.commands.len());
                    self.commands.push(Command::Open(0));
                }
                ']' => {
                    match self.opens.pop() {
                        Some(val) => {
                            self.commands[val] = Command::Open(self.commands.len());
                            self.commands.push(Command::Close(val));
                        }
                        None => return Err(CompileError::UnmatchedBracket(i)),
                    }
                }
                ',' => self.commands.push(Command::Read),
                '.' => self.commands.push(Command::Write),
                _ => {}
            }
        }

        if self.opens.len() > 0 {
            return Err(CompileError::UnmatchedBracket(self.opens[0]));
        }

        self.commands.push(Command::Halt);
        Ok(Program(self.commands))
    }
}
