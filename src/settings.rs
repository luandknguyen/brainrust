/// EOF behavior
#[derive(Clone)]
pub enum EofBehavior {
    AsIs,
    NegativeOne,
    Zero,
}

/// Newline modes
#[derive(Clone)]
pub enum NewlineMode {
    CRLF,
    LF,
}

/// Input ascii or digit
#[derive(Clone)]
pub enum InputMode {
    Ascii,
    Digit,
}

/// Settings
#[derive(Clone)]
pub struct Settings {
    pub dynamic_size: bool,
    pub array_size: usize,
    pub eof_behavior: EofBehavior,
    pub newline_mode: NewlineMode,
    pub ignore_newline: bool,
    pub input_mode: InputMode,
    pub wrapping: bool,
}