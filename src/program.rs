/// List of available BF commands. Currently, only a subset of BF commands are supported.
#[derive(Clone, Debug)]
pub enum Command {
    Halt,         // halt program
    Right,        // increment pointer
    Left,         // decrement pointer
    Inc,          // increment byte at pointer
    Dec,          // decrement byte at pointer
    Open(usize),  // if byte at pointer = 0, jump to usize (corresponding Close)
    Close(usize), // if byte at pointer != 0, jump back to usize (corresponding Open)
    Read,         // read from either stdin or file
    Write,        // write to either stdout or file
}

/// Instructions for the program
#[derive(Clone)]
pub struct Program(pub Vec<Command>);
