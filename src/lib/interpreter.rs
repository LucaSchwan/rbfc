use std::io::Read;

use crate::lexer::{Token, TokenType};
use crate::parser::{Parser, ParserError};
use thiserror::Error;

/// Error type for the interpreter
///
/// This error type is used to represent the different kinds of errors that can occur during the
/// interpretation
#[derive(Debug, Error, PartialEq)]
pub enum InterpreterError {
    #[error("Unexpected none size at {0}")]
    UnexpectedNoneSize(usize),
    #[error("Tape underflow at {0}")]
    TapeUnderflow(usize),
    #[error("Tape overflow at {0}")]
    TapeOverflow(usize),
    #[error("Unexpected input error")]
    InputError,
    #[error("Parsing error: {0}")]
    ParserError(ParserError),
}

/// The interpreter struct
///
/// This struct is used to represent the interpreter. It contains the tape, the operations
/// and the program counter. It also contains the data pointer and the settings for the interpreter
/// such as the ascii flag
///
/// # Fields
/// * `tape` - The tape for the program
/// * `ops` - The operations for the program
/// * `pc` - The program counter
/// * `dp` - The data pointer
/// * `settings` - The settings for the interpreter
///
/// # Example
/// ```
/// use rbfc::interpreter::{Interpreter, InterpreterError};
///
/// fn main() -> Result<(), InterpreterError> {
///    let input = String::from("+++.>+++.>,.>,.");
///    let mut interpreter = Interpreter::new(input, vec![3, 3])?;
///    interpreter.interpret().unwrap();
///    Ok(())
/// }
/// ```
pub struct Interpreter {
    tape: [u8; 30000],
    ops: Vec<Token>,
    pc: usize,
    dp: usize,
}

impl Interpreter {
    /// Create a new instance of the interpreter
    ///
    /// # Arguments
    /// * `code` - A string that contains the code to be interpreted
    ///
    /// # Example
    /// ```
    /// use rbfc::interpreter::{Interpreter};
    ///
    /// let input = String::from("+++.>+++.>,.>,.");
    /// let mut interpreter = Interpreter::new(input, vec![3, 3]).unwrap();
    /// ```
    pub fn new(code: String) -> Result<Interpreter, InterpreterError> {
        let mut parser = Parser::new(code);
        let ops = match parser.parse() {
            Ok(ops) => ops,
            Err(e) => return Err(InterpreterError::ParserError(e)),
        };
        Ok(Interpreter {
            tape: [u8::default(); 30000],
            ops,
            pc: 0,
            dp: 0,
        })
    }

    /// Execute the operations
    ///
    /// This method is used to execute the operations. It iterates over the operations and executes
    /// them. It returns a Result with the unit type or an InterpreterError
    ///
    /// # Example
    /// ```
    /// use rbfc::interpreter::{Interpreter};
    ///
    /// let input = String::from("+++.>+++.>,.>,.");
    /// let mut interpreter = Interpreter::new(input).unwrap();
    /// interpreter.interpret().unwrap();
    /// ```
    pub fn interpret(&mut self) -> Result<(), InterpreterError> {
        while self.pc < self.ops.len() {
            let op = &self.ops[self.pc];
            match op.token_type {
                TokenType::Eof => break,
                TokenType::Plus => {
                    if let Some(size) = op.size {
                        self.tape[self.dp] = self.tape[self.dp].wrapping_add(size as u8);
                    } else {
                        return Err(InterpreterError::UnexpectedNoneSize(op.loc));
                    }
                }
                TokenType::Minus => {
                    if let Some(size) = op.size {
                        self.tape[self.dp] = self.tape[self.dp].wrapping_sub(size as u8);
                    } else {
                        return Err(InterpreterError::UnexpectedNoneSize(op.loc));
                    }
                }
                TokenType::ShiftRight => {
                    if let Some(size) = op.size {
                        if self.dp + size < 30000 {
                            self.dp += size;
                        } else {
                            return Err(InterpreterError::TapeOverflow(op.loc));
                        }
                    } else {
                        return Err(InterpreterError::UnexpectedNoneSize(op.loc));
                    }
                }
                TokenType::ShiftLeft => {
                    if let Some(size) = op.size {
                        if self.dp >= size {
                            self.dp -= size;
                        } else {
                            return Err(InterpreterError::TapeUnderflow(op.loc));
                        }
                    } else {
                        return Err(InterpreterError::UnexpectedNoneSize(op.loc));
                    }
                }
                TokenType::Dot => {
                    let op = &self.ops[self.pc];
                    match op.size {
                        Some(size) => {
                            for _ in 0..size {
                                print!("{}", self.tape[self.dp] as char);
                            }
                        }
                        None => return Err(InterpreterError::UnexpectedNoneSize(op.loc)),
                    }
                }
                TokenType::Comma => {
                    if let Some(size) = op.size {
                        for _ in 0..size {
                            let c = std::io::stdin()
                                .bytes()
                                .next()
                                .and_then(|result| result.ok());
                            match c {
                                Some(c) => self.tape[self.dp] = c,
                                None => return Err(InterpreterError::InputError),
                            }
                        }
                    } else {
                        return Err(InterpreterError::UnexpectedNoneSize(op.loc));
                    }
                }
                TokenType::OpenBracket => {
                    if self.tape[self.dp] == 0 {
                        if let Some(size) = op.size {
                            self.pc = size;
                        } else {
                            let op = &self.ops[self.pc];
                            return Err(InterpreterError::UnexpectedNoneSize(op.loc));
                        }
                    }
                }
                TokenType::CloseBracket => {
                    if self.tape[self.dp] != 0 {
                        if let Some(size) = op.size {
                            self.pc = size;
                        } else {
                            let op = &self.ops[self.pc];
                            return Err(InterpreterError::UnexpectedNoneSize(op.loc));
                        }
                    }
                }
            }
            self.pc += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpreter() {
        let input = String::from("++[->+<]");
        let mut interpreter = Interpreter::new(input).unwrap();
        interpreter.interpret().unwrap();
    }
}
