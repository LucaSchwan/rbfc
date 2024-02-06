use clap::Parser;
use rbfc::interpreter::{Interpreter, InterpreterError};
use std::path::PathBuf;
use thiserror::Error;

extern crate rbfc;

/// The arguments for the program
#[derive(Parser, Debug)]
struct Args {
    /// The file to interpret
    file: PathBuf,
    /// Input as a list of bytes separated by commas
    #[arg(short, long)]
    bytes: Option<String>,
    #[arg(short, long)]
    /// Input as a list of decimal numbers separated by commas
    dec: Option<String>,
}

/// The error type for the program
#[derive(Error, Debug)]
enum RBFCError {
    #[error("Error reading file: {0}")]
    ReadingFile(String),
    #[error("Error parsing input")]
    ParsingInput,
    #[error("Error while interpreting: {0}")]
    Interpreter(InterpreterError),
}

fn main() -> Result<(), RBFCError> {
    let args = Args::parse();
    let file = args.file.to_string_lossy().to_string();
    let code = std::fs::read_to_string(file.clone()).or(Err(RBFCError::ReadingFile(file)))?;

    let input: Vec<u8> = match args.bytes {
        Some(bytes) => bytes
            .split(',')
            .map(|x| u8::from_str_radix(x, 16).or(Err(RBFCError::ParsingInput)))
            .collect::<Result<Vec<u8>, RBFCError>>()?,
        None => match args.dec {
            Some(dec) => dec
                .split(',')
                .map(|x| x.parse::<u8>().or(Err(RBFCError::ParsingInput)))
                .collect::<Result<Vec<u8>, RBFCError>>()?,
            None => vec![],
        },
    };

    let mut interpreter = match Interpreter::new(code, input) {
        Ok(i) => i,
        Err(e) => return Err(RBFCError::Interpreter(e)),
    };

    match interpreter.interpret() {
        Ok(()) => Ok(()),
        Err(e) => Err(RBFCError::Interpreter(e)),
    }
}
