//! This module is the main module of the project. It contains the interpreter, lexer and parser modules.
//!
//! # Parser example
//! ```
//! extern crate rbfc;
//!
//! use rbfc::parser;
//!
//! fn main() {
//!     let input = String::from("+++[->+<]...,,,");
//!     let mut parser = parser::Parser::new(input);
//!     let mut ops = parser.parse().unwrap();
//!     while let Some(op) = ops.pop() {
//!         println!("{:?}", op);
//!     }
//! }
//! ```
//!
//! # Lexer example
//! ```
//! use rbfc::lexer::{Lexer, Token, TokenType};
//!
//! let mut lexer = Lexer::new(String::from("+++"));
//! assert_eq!(
//!    lexer.next_token(),
//!    Token {
//!    token_type: TokenType::Plus,
//!    size: Some(3),
//!    loc: 0
//! });
//! ```

pub mod compiler;
pub mod interpreter;
pub mod lexer;
pub mod parser;
