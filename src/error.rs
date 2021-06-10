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

/// Result of running
#[derive(Debug)]
pub enum RunResult {
    None,
    Halted,
    /// Index out of state's bound
    ///
    /// `usize` is the indexing location
    IndexOutOfBound(usize),
    ReadFailed,
    WriteFailed,
    ParseNumError,
}
