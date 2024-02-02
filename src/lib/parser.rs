use thiserror::Error;

use super::lexer;

#[derive(Debug, Error, PartialEq)]
pub enum ParserError {
    #[error("Unmatched bracket at position {0}")]
    UnmatchedBracket(usize),
    #[error("Unexpected end of file at position {0}, expected closing bracket at position {1}")]
    UnexpectedEof(usize, usize),
}

/// The Parser struct is responsible for parsing a sequence of tokens
/// into a sequence of operations.
///
/// # Example
/// ```
/// use rbfc::parser::Parser;
///
/// let input = String::from("+++[->+<]...,,,");
/// let mut parser = Parser::new(input);
/// let mut ops = parser.parse().unwrap();
/// ```
#[derive(Debug)]
pub struct Parser {
    lexer: lexer::Lexer,
}

impl Parser {
    /// Create a new parser from a string
    ///
    /// # Arguments
    /// * `input` - A string to be parsed
    ///
    /// # Example
    /// ```
    /// use rbfc::parser::Parser;
    ///
    /// let input = String::from("+++[->+<]...,,,");
    /// let mut parser = Parser::new(input);
    /// ```
    pub fn new(input: String) -> Parser {
        Parser {
            lexer: lexer::Lexer::new(input),
        }
    }

    /// Parse the input string into a sequence of operations
    ///
    /// # Example
    /// ```
    /// use rbfc::parser::Parser;
    ///
    /// let input = String::from("+++[->+<]...,,,");
    /// let mut parser = Parser::new(input);
    /// let mut ops = parser.parse().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns an error if the input string contains unmatched brackets
    /// or if the input string ends unexpectedly
    /// ```
    /// use rbfc::parser::Parser;
    /// use rbfc::parser::ParserError;
    ///
    /// let input = String::from("+++[->+<");
    /// let mut parser = Parser::new(input);
    /// let result = parser.parse();
    /// assert_eq!(result, Err(ParserError::UnexpectedEof(6, 1)));
    /// ```
    pub fn parse(&mut self) -> Result<Vec<lexer::Token>, ParserError> {
        let mut jump_stack = Vec::new();
        let mut ops = Vec::new();
        let mut loc = 0;

        loop {
            let mut token = self.lexer.next_token();
            match token.token_type {
                lexer::TokenType::Eof => break,
                lexer::TokenType::OpenBracket => {
                    jump_stack.push(loc);
                    ops.push(token);
                }
                lexer::TokenType::CloseBracket => {
                    let jump = jump_stack.pop().ok_or(ParserError::UnmatchedBracket(loc))?;
                    token.size = Some(jump);
                    ops[jump].size = Some(loc + 1);
                    ops.push(token);
                }
                _ => {
                    ops.push(token);
                }
            }
            loc += 1;
        }

        if !jump_stack.is_empty() {
            return Err(ParserError::UnexpectedEof(
                loc,
                jump_stack.pop().expect("Should be some location"),
            ));
        }

        Ok(ops)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let input = String::from("++[->+<]");
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();
        assert_eq!(
            result,
            vec![
                lexer::Token {
                    token_type: lexer::TokenType::Plus,
                    size: Some(2),
                    loc: 0
                },
                lexer::Token {
                    token_type: lexer::TokenType::OpenBracket,
                    size: Some(7),
                    loc: 2
                },
                lexer::Token {
                    token_type: lexer::TokenType::Minus,
                    size: Some(1),
                    loc: 3
                },
                lexer::Token {
                    token_type: lexer::TokenType::ShiftRight,
                    size: Some(1),
                    loc: 4
                },
                lexer::Token {
                    token_type: lexer::TokenType::Plus,
                    size: Some(1),
                    loc: 5
                },
                lexer::Token {
                    token_type: lexer::TokenType::ShiftLeft,
                    size: Some(1),
                    loc: 6
                },
                lexer::Token {
                    token_type: lexer::TokenType::CloseBracket,
                    size: Some(1),
                    loc: 7
                },
            ]
        );
    }

    #[test]
    fn test_parser_unmatched_bracket() {
        let input = String::from("++[->+<");
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert_eq!(result, Err(ParserError::UnexpectedEof(6, 1)));
    }

    #[test]
    fn test_parser_unexpected_eof() {
        let input = String::from("++->+<]");
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert_eq!(result, Err(ParserError::UnmatchedBracket(5)));
    }
}
