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

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
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

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer { input, position: 0 }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.position);
        self.position += 1;
        c
    }

    pub fn next_token(&mut self) -> Token {
        if self.position >= self.input.len() {
            return Token {
                token_type: TokenType::Eof,
                size: None,
            };
        }

        let mut c = self.next_char().unwrap();

        while Token::is_token(&c).is_none() {
            c = self.next_char().unwrap();
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
