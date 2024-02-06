use clap::Parser;
use rbfc::{
    compiler::{Compiler, CompilerError},
    interpreter::{Interpreter, InterpreterError},
};
use std::path::PathBuf;
use thiserror::Error;

extern crate rbfc;

/// The arguments for the program
#[derive(Parser, Debug)]
struct Args {
    /// The file to interpret
    file_path: PathBuf,

    /// Input as a list of bytes separated by commas
    #[arg(short, long)]
    bytes: Option<String>,

    /// Input as a list of decimal numbers separated by commas
    #[arg(short, long)]
    dec: Option<String>,

    /// The output folder
    #[arg(short, long)]
    output: Option<String>,

    /// Whether to interpret the file
    #[arg(short, long)]
    interpret: bool,
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
    #[error("Error while compiling: {0}")]
    Compiler(CompilerError),
    #[error("Error writing file: {0}")]
    WritingFile(String),
}

fn main() -> Result<(), RBFCError> {
    let args = Args::parse();
    let file_name = args
        .file_path
        .file_name()
        .ok_or(RBFCError::ReadingFile("Couldn't get filename".to_string()))?
        .to_os_string()
        .into_string()
        .or(Err(RBFCError::ReadingFile(
            "Couldn't get filename".to_string(),
        )))?;

    let code = std::fs::read_to_string(args.file_path)
        .or(Err(RBFCError::ReadingFile(file_name.clone())))?;

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

    if args.interpret {
        let mut interpreter = match Interpreter::new(code, input) {
            Ok(i) => i,
            Err(e) => return Err(RBFCError::Interpreter(e)),
        };

        match interpreter.interpret() {
            Ok(()) => return Ok(()),
            Err(e) => return Err(RBFCError::Interpreter(e)),
        }
    } else {
        let compiler = match Compiler::new(code) {
            Ok(c) => c,
            Err(e) => return Err(RBFCError::Compiler(e)),
        };

        match compiler.compile_code() {
            Ok(asm) => {
                let file = if let Some(output) = args.output {
                    format!("{}/{}", output, file_name.replace(".bf", ".asm"))
                } else {
                    file_name.replace(".bf", ".asm").to_string()
                };
                std::fs::write(file.clone(), asm).or(Err(RBFCError::WritingFile(file)))?;
            }
            Err(e) => return Err(RBFCError::Compiler(e)),
        }
    }
    Ok(())
}
