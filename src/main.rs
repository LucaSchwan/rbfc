use clap::Parser;
use rbfc::interpreter::{Interpreter, InterpreterError, InterpreterSettings};
use std::path::PathBuf;
use thiserror::Error;

mod rbfc;

#[derive(Parser, Debug)]
struct Args {
    file: PathBuf,
    #[arg(short, long)]
    bytes: Option<String>,
    #[arg(short, long)]
    dec: Option<String>,
    #[arg(short, long)]
    ascii: bool,
}

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

    let mut settings: InterpreterSettings = Default::default();

    if args.ascii {
        settings.ascii = true;
    }

    let mut interpreter = match Interpreter::new(code, input, settings) {
        Ok(i) => i,
        Err(e) => return Err(RBFCError::Interpreter(e)),
    };

    match interpreter.interpret() {
        Ok(()) => Ok(()),
        Err(e) => Err(RBFCError::Interpreter(e)),
    }
}
