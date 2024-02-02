/// The lexer module is responsible for tokenizing the input string
/// into a sequence of tokens.
///
/// # Example
/// ```
/// use rbfc::lexer::{Lexer, Token, TokenType};
/// let input = String::from("+++[->+<]...,,,");
/// let mut lexer = Lexer::new(input);
/// let token = lexer.next_token();
/// assert_eq!(token, Token { token_type: TokenType::Plus, size: Some(3) });
/// ```

/// The TokenType enum represents the different types of tokens
/// that the lexer can produce.
///
/// # Example
/// ```
/// use rbfc::lexer::TokenType;
/// assert_eq!(TokenType::Eof, TokenType::Eof);
/// ```
///
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Eof,
    ShiftLeft,
    ShiftRight,
    Plus,
    Minus,
    Dot,
    Comma,
    OpenBracket,
    CloseBracket,
}

/// The Token struct represents a single token produced by the lexer.
/// It contains the token type and the size of the token if applicable.
/// The size is the number of consecutive tokens of the same type.
/// For example, the token "+++" would have a size of 3.
/// The size is None for tokens that are not repeated.
#[derive(Debug, PartialEq)]
pub struct Token {
    /// The type of the token
    pub token_type: TokenType,
    /// The size of the token
    pub size: Option<usize>,
}

impl Token {
    pub fn is_token(c: &char) -> Option<TokenType> {
        match c {
            '<' => Some(TokenType::ShiftLeft),
            '>' => Some(TokenType::ShiftRight),
            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            '.' => Some(TokenType::Dot),
            ',' => Some(TokenType::Comma),
            '[' => Some(TokenType::OpenBracket),
            ']' => Some(TokenType::CloseBracket),
            _ => None,
        }
    }
}

/// The Lexer struct is responsible for tokenizing the input string
/// into a sequence of tokens.
#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    /// Create a new lexer from a string
    ///
    /// # Arguments
    /// * `input` - A string to be tokenized
    ///
    /// # Example
    /// ```
    /// use rbfc::lexer::Lexer;
    ///
    /// let input = String::from("+++[->+<]...,,,");
    /// let mut lexer = Lexer::new(input);
    /// ```
    pub fn new(input: String) -> Lexer {
        Lexer { input, position: 0 }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.position);
        self.position += 1;
        c
    }

    /// Get the next token from the input
    ///
    /// # Example
    /// ```
    /// use rbfc::lexer::{Lexer, Token, TokenType};
    ///
    /// let mut lexer = Lexer::new(String::from("+++"));
    /// assert_eq!(
    ///    lexer.next_token(),
    ///    Token {
    ///    token_type: TokenType::Plus,
    ///    size: Some(3)
    /// });
    /// ```
    pub fn next_token(&mut self) -> Token {
        let mut c = char::default();

        while Token::is_token(&c).is_none() {
            c = match self.next_char() {
                Some(c) => c,
                None => {
                    return Token {
                        token_type: TokenType::Eof,
                        size: None,
                    }
                }
            };
        }

        let token_type = Token::is_token(&c).expect("Should be some token_type");

        match token_type {
            TokenType::Dot
            | TokenType::Comma
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::ShiftLeft
            | TokenType::ShiftRight => {
                let mut size = 1;

                while let Some(next_char) = self.next_char() {
                    if let Some(next_token_type) = Token::is_token(&next_char) {
                        if next_token_type == token_type {
                            size += 1;
                        } else {
                            break;
                        }
                    }
                }

                self.position -= 1;

                Token {
                    token_type,
                    size: Some(size),
                }
            }
            _ => Token {
                token_type,
                size: None,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let mut lexer = Lexer::new(String::from("+++"));
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Plus,
                size: Some(3)
            }
        );

        let mut lexer = Lexer::new(String::from("++>"));
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Plus,
                size: Some(2)
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::ShiftRight,
                size: Some(1)
            }
        );
    }

    #[test]
    fn test_brackets() {
        let mut lexer = Lexer::new(String::from("["));
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::OpenBracket,
                size: None
            }
        );

        let mut lexer = Lexer::new(String::from("]"));
        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::CloseBracket,
                size: None
            }
        );
    }
}
