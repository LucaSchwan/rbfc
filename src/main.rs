use clap::Parser;
use rbfc::{
    compiler::{Compiler, CompilerError},
    interpreter::{Interpreter, InterpreterError, InterpreterSettings},
};
use std::path::PathBuf;
use thiserror::Error;
extern crate log;
extern crate pretty_env_logger;

extern crate rbfc;

/// The arguments for the program
#[derive(Parser, Debug)]
struct Args {
    /// The file to interpret
    file_path: PathBuf,

    /// The output folder
    #[arg(short, long)]
    output: Option<String>,

    /// Whether to interpret the file
    #[arg(short, long)]
    interpret: bool,

    /// Whether to wrap the tape
    #[arg(short, long)]
    wrap: bool,
}

/// The error type for the program
#[derive(Error, Debug)]
enum RBFCError {
    #[error("Error reading file: {0}")]
    ReadingFile(String),
    #[error("Error while interpreting: {0}")]
    Interpreter(InterpreterError),
    #[error("Error while compiling: {0}")]
    Compiler(CompilerError),
    #[error("Error writing file: {0}")]
    WritingFile(String),
}

fn main() -> Result<(), RBFCError> {
    pretty_env_logger::init();

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

    if args.interpret {
        let settings = InterpreterSettings { wrap: args.wrap };
        let mut interpreter = match Interpreter::new(code, settings) {
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
