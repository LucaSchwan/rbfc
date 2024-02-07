use crate::lexer::{Token, TokenType};
use crate::parser::{Parser, ParserError};
use log::{debug, trace};
use std::io::Read;
use thiserror::Error;

/// Error type for the interpreter
///
/// This error type is used to represent the different kinds of errors that can occur during the
/// interpretation
#[derive(Debug, Error, PartialEq)]
pub enum InterpreterError {
    #[error("Unexpected none size at {0}")]
    UnexpectedNoneSize(usize),
    #[error("Unexpected input error")]
    InputError,
    #[error("Parsing error: {0}")]
    ParserError(ParserError),
    #[error("Tape overflow at {0}")]
    TapeOverflow(usize),
    #[error("Tape underflow at {0}")]
    TapeUnderflow(usize),
}

/// The settings for the interpreter
///
/// This struct is used to represent the settings for the interpreter. It contains the wrap
/// setting which is used to determine whether the tape should wrap around
/// or not
///
/// # Fields
/// * `wrap` - Whether the tape should wrap around or not
///
/// # Example
/// ```
/// use rbfc::interpreter::{InterpreterSettings};
/// let settings = InterpreterSettings { wrap: true };
/// ```
#[derive(Debug, Default)]
pub struct InterpreterSettings {
    pub wrap: bool,
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
/// use rbfc::interpreter::{Interpreter, InterpreterSettings};
///
/// let input = String::from("+++.>+++.>.>.");
/// let mut interpreter = Interpreter::new(input, InterpreterSettings::default()).unwrap();
/// interpreter.interpret().unwrap();
/// ```
///
/// # Errors
/// Returns an error if the input string contains unmatched brackets
/// or if the input string ends unexpectedly
/// ```
/// use rbfc::interpreter::{Interpreter, InterpreterSettings, InterpreterError};
/// use rbfc::parser::ParserError;
///
/// let mut interpreter = Interpreter::new("+++[->+<".to_string(), InterpreterSettings::default());
/// matches!(interpreter, Err(InterpreterError::ParserError(ParserError::UnexpectedEof(6, 1))));
/// ```
pub struct Interpreter {
    tape: [u8; 30000],
    ops: Vec<Token>,
    pc: usize,
    dp: usize,
    settings: InterpreterSettings,
}

impl Interpreter {
    /// Create a new instance of the interpreter
    ///
    /// # Arguments
    /// * `code` - A string that contains the code to be interpreted
    ///
    /// # Example
    /// ```
    /// use rbfc::interpreter::{Interpreter, InterpreterSettings};
    ///
    /// let input = String::from("+++.>+++.>.>.");
    /// let mut interpreter = Interpreter::new(input, InterpreterSettings::default()).unwrap();
    /// ```
    pub fn new(
        code: String,
        settings: InterpreterSettings,
    ) -> Result<Interpreter, InterpreterError> {
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
            settings,
        })
    }

    /// Execute the operations
    ///
    /// This method is used to execute the operations. It iterates over the operations and executes
    /// them. It returns a Result with the unit type or an InterpreterError
    ///
    /// # Example
    /// ```
    /// use rbfc::interpreter::{Interpreter, InterpreterSettings};
    ///
    /// let input = String::from("+++.>+++.>.>.");
    /// let mut interpreter = Interpreter::new(input, InterpreterSettings::default()).unwrap();
    /// interpreter.interpret().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns an error if the input string contains unmatched brackets
    /// or if the input string ends unexpectedly
    /// ```
    /// use rbfc::interpreter::{Interpreter, InterpreterSettings, InterpreterError};
    /// use rbfc::parser::ParserError;
    ///
    /// let mut interpreter = Interpreter::new("+++[->+<".to_string(), InterpreterSettings::default());
    /// matches!(interpreter, Err(InterpreterError::ParserError(ParserError::UnexpectedEof(6, 1))));
    /// ```
    pub fn interpret(&mut self) -> Result<(), InterpreterError> {
        while self.pc < self.ops.len() {
            let op = &self.ops[self.pc];
            trace!("Tape: {:?}", self.tape[0..10].to_vec());
            match op.token_type {
                TokenType::Eof => break,
                TokenType::Plus => {
                    if let Some(size) = op.size {
                        self.tape[self.dp] = self.tape[self.dp].wrapping_add(size as u8);
                    } else {
                        return Err(InterpreterError::UnexpectedNoneSize(op.loc));
                    }
                    debug!(
                        "Plus: (loc: {loc}, dp: {dp}, tape: {tape})",
                        loc = op.loc,
                        dp = self.dp,
                        tape = self.tape[self.dp]
                    );
                }
                TokenType::Minus => {
                    if let Some(size) = op.size {
                        self.tape[self.dp] = self.tape[self.dp].wrapping_sub(size as u8);
                    } else {
                        return Err(InterpreterError::UnexpectedNoneSize(op.loc));
                    }
                    debug!(
                        "Minus: (loc: {loc}, dp: {dp}, tape: {tape})",
                        loc = op.loc,
                        dp = self.dp,
                        tape = self.tape[self.dp]
                    );
                }
                TokenType::ShiftRight => {
                    if let Some(size) = op.size {
                        if self.dp + size >= self.tape.len() {
                            if self.settings.wrap {
                                self.dp = self.dp + size - self.tape.len();
                            } else {
                                return Err(InterpreterError::TapeOverflow(op.loc));
                            }
                        } else {
                            self.dp += size;
                        }
                    } else {
                        return Err(InterpreterError::UnexpectedNoneSize(op.loc));
                    }
                    debug!(
                        "ShiftRight: (loc: {loc}, dp: {dp}, tape: {tape})",
                        loc = op.loc,
                        dp = self.dp,
                        tape = self.tape[self.dp]
                    );
                }
                TokenType::ShiftLeft => {
                    if let Some(size) = op.size {
                        if self.dp < size {
                            if self.settings.wrap {
                                self.dp += self.tape.len() - (size - self.dp);
                            } else {
                                return Err(InterpreterError::TapeUnderflow(op.loc));
                            }
                        } else {
                            self.dp -= size;
                        }
                    } else {
                        return Err(InterpreterError::UnexpectedNoneSize(op.loc));
                    }
                    debug!(
                        "ShiftLeft: (loc: {loc}, dp: {dp}, tape: {tape})",
                        loc = op.loc,
                        dp = self.dp,
                        tape = self.tape[self.dp]
                    );
                }
                TokenType::Dot => {
                    debug!(
                        "Dot: (loc: {loc}, dp: {dp}, tape: {tape})",
                        loc = op.loc,
                        dp = self.dp,
                        tape = self.tape[self.dp]
                    );
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
                    debug!(
                        "Comma: (loc: {loc}, dp: {dp}, tape: {tape})",
                        loc = op.loc,
                        dp = self.dp,
                        tape = self.tape[self.dp]
                    );
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
                    debug!(
                        "OpenBracket: (loc: {loc}, dp: {dp}, tape: {tape})",
                        loc = op.loc,
                        dp = self.dp,
                        tape = self.tape[self.dp]
                    );
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
                    debug!(
                        "CloseBracket: (loc: {loc}, dp: {dp}, tape: {tape})",
                        loc = op.loc,
                        dp = self.dp,
                        tape = self.tape[self.dp]
                    );
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
        let settings: InterpreterSettings = Default::default();
        let mut interpreter = Interpreter::new(input, settings).unwrap();
        interpreter.interpret().unwrap();
    }
}
