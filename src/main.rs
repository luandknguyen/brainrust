use std::fs::File;
use std::io::prelude::*;

use brainfuck_rs::*;

use clap::{App, Arg, ArgMatches};

/// Error while parsing command-line arguments
enum ParseError {
    ArraySize(String),
    EofBehavior(String),
    InputMode(String),
    NewlineMode(String),
}

/// Read file and return its contents
fn read_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Parsing command-line arguments into settings
fn settings<'a>(args: &ArgMatches<'a>) -> Result<Settings, ParseError> {
    let array_size = args.value_of("array_size").unwrap_or(&"30000");
    let array_size = match array_size.parse::<usize>() {
        Ok(val) => val,
        Err(_) => return Err(ParseError::ArraySize(String::from(array_size))),
    };

    let eof_behavior = args.value_of("eof_behavior").unwrap_or(&"as_is");
    let eof_behavior = match eof_behavior {
        "as_is" => EofBehavior::AsIs,
        "negative_one" => EofBehavior::NegativeOne,
        "zero" => EofBehavior::Zero,
        _ => return Err(ParseError::EofBehavior(String::from(eof_behavior))),
    };

    let newline_mode = args.value_of("newline").unwrap_or(&"CRLF");
    let newline_mode = match newline_mode {
        "CRLF" => NewlineMode::CRLF,
        "LF" => NewlineMode::LF,
        _ => return Err(ParseError::NewlineMode(String::from(newline_mode))),
    };

    let input_mode = args.value_of("input_mode").unwrap_or(&"ascii");
    let input_mode = match input_mode {
        "ascii" => InputMode::Ascii,
        "digit" => InputMode::Digit,
        _ => return Err(ParseError::InputMode(String::from(input_mode))),
    };

    Ok(Settings {
        dynamic_size: args.is_present("dynamic_size"),
        array_size,
        eof_behavior,
        newline_mode,
        ignore_newline: args.is_present("ignore_newline") || !args.is_present("input"),
        input_mode,
        wrapping: args.is_present("wrapping"),
    })
}

fn main() -> std::io::Result<()> {
    let app = App::new("brainfuck_rs")
        .author("Luan N.")
        .version("0.1")
        .about("An interpreter written in Rust.")
        .args(&[
            Arg::with_name("source")
                .required(true)
                .takes_value(true)
                .value_name("SOURCE")
                .help("Source file"),
            Arg::with_name("input")
                .short("i")
                .takes_value(true)
                .value_name("INPUT")
                .help("Specify which file to read input from. Default: stdin."),
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .value_name("OUTPUT")
                .help("Specify which file to write output to. Default: stdout."),
            Arg::with_name("dynamic_size")
                .short("d")
                .long("dynamic_size")
                .help("Use dynamic size instead of fixed size array. If this flag is used, --array_size will specify the initial size."),
            Arg::with_name("array_size")
                .short("s")
                .long("array_size")
                .takes_value(true)
                .value_name("SIZE")
                .help("Size of array. Default: 30000."),
            Arg::with_name("final_array")
                .long("final_array")
                .help("Display final array after program finished."),
            Arg::with_name("eof_behavior")
                .long("eof_behavior")
                .takes_value(true)
                .value_name("EOF_BEHAVIOR")
                .help(
                    "\
                    Behavior when received EOF as input:
                        as_is [Default] -- leave the current cell untouched.
                        zero -- set the current cell value to 0.
                        negative_one -- set the current cell value to -1.
                    ",
                ),
            Arg::with_name("newline_mode")
                .long("newline_mode")
                .takes_value(true)
                .value_name("NEWLINE_MODE")
                .help("Select newline mode: CRLF or LF. Default: CRLF"),
            Arg::with_name("ignore_newline")
                .long("ignore_newline")
                .help("Ignore newline input character. Default to true if input is stdin, false otherwise."),
            Arg::with_name("input_mode")
                .long("input_mode")
                .takes_value(true)
                .value_name("INPUT_MODE")
                .help(
                    "\
                    Select input mode:
                        ascii [Default] -- no input conversion.
                        digit -- convert input into digit (between 0 ... 255)
                    ",
                ),
            Arg::with_name("wrapping")
                .long("wrapping")
                .short("w")
                .help("Wrapping '>' and '<'. \"--dynamic_size\" will override this flag.")
        ]);
    let args = app.get_matches();

    let settings = match settings(&args) {
        Ok(settings) => settings,
        Err(ParseError::ArraySize(err)) => {
            eprintln!("Failed to parse array_size: \"{}\"", err);
            return Ok(());
        }
        Err(ParseError::EofBehavior(err)) => {
            eprintln!("Failed to parse eof_behavior: \"{}\"", err);
            return Ok(());
        }
        Err(ParseError::NewlineMode(err)) => {
            eprintln!("Failed to parse newline_mode: \"{}\"", err);
            return Ok(());
        }
        Err(ParseError::InputMode(err)) => {
            eprintln!("Failed to parse input_mode: \"{}\"", err);
            return Ok(());
        }
    };

    let src = read_file(args.value_of("source").unwrap())?;

    let mut interpreter = match InterpreterBuilder::new(src, settings)
        .reader(args.value_of("input"))?
        .writer(args.value_of("output"))?
        .build()
    {
        Ok(interpreter) => interpreter,
        Err(CompileError::Syntax(loc, ch)) => {
            eprintln!("Invalid syntax at {}: '{}'", loc, ch);
            return Ok(());
        }
        Err(CompileError::UnmatchedBracket(loc)) => {
            eprintln!("Cannot find matching bracket at {}", loc);
            return Ok(());
        }
    };

    let mut state = interpreter.ready();
    loop {
        let result = interpreter.next(&mut state);
        match result {
            RunResult::None => continue,
            RunResult::Halted => break,
            RunResult::IndexOutOfBound(idx) => {
                eprintln!("Index out of bound: {}", idx);
                break;
            }
            RunResult::ReadFailed => {
                eprintln!("Failed to read");
                break;
            }
            RunResult::WriteFailed => {
                eprintln!("Failed to write");
                break;
            }
            RunResult::ParseNumError => {
                eprintln!("Failed to parse input into number");
                break;
            }
        };
    }

    if args.is_present("final_array") {
        println!("\nFinal array: {:?}", state.cells.0)
    }

    Ok(())
}
