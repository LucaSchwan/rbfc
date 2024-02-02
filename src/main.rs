mod lexer {
    #[derive(Debug)]
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
        pub operand: Option<u8>,
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
            if self.position >= self.input.len() {
                return None;
            }

            let char = self.input.chars().nth(self.position);
            self.position += 1;
            char
        }

        pub fn next_token(&mut self) -> Token {
            let mut next_char = match self.next_char() {
                Some(c) => c,
                None => {
                    return Token {
                        token_type: TokenType::Eof,
                        operand: None,
                    }
                }
            };

            while Token::is_token(&next_char).is_none() {
                next_char = match self.next_char() {
                    Some(c) => c,
                    None => {
                        return Token {
                            token_type: TokenType::Eof,
                            operand: None,
                        }
                    }
                };
            }

            let token_type = Token::is_token(&next_char).expect("Should be a token");

            Token {
                token_type,
                operand: None,
            }
        }
    }
}

fn main() {
    let input = String::from("+++");
    let mut lexer = lexer::Lexer::new(input);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if matches!(token.token_type, lexer::TokenType::Eof) {
            break;
        }
    }
}
