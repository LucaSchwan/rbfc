use super::lexer;

#[derive(Debug)]
pub struct Parser {
    lexer: lexer::Lexer,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            lexer: lexer::Lexer::new(input),
        }
    }

    pub fn parse(&mut self) -> Vec<lexer::Token> {
        let mut jump_stack = Vec::new();
        let mut ops = Vec::new();
        let mut curr_op = 0;

        loop {
            let mut token = self.lexer.next_token();
            match token.token_type {
                lexer::TokenType::Eof => break,
                lexer::TokenType::OpenBracket => {
                    jump_stack.push(curr_op);
                    ops.push(token);
                }
                lexer::TokenType::CloseBracket => {
                    let jump = jump_stack.pop().expect("No matching open bracket");
                    token.size = Some(jump);
                    ops[jump].size = Some(curr_op);
                    ops.push(token);
                }
                _ => {
                    ops.push(token);
                }
            }
            curr_op += 1;
        }

        ops
    }
}
