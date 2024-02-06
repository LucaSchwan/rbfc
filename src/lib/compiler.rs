use crate::lexer::{Token, TokenType};
use crate::parser::{Parser, ParserError};
use indoc::{formatdoc, indoc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Parsing error: {0}")]
    ParsingError(ParserError),
    #[error("Unexpected none size at {0}")]
    UnexpectedNoneSize(usize),
    #[error("Unexpected end of file")]
    UnexpectedEof,
    #[error("Unmatched bracket at position {0}")]
    UnmatchedBracket(usize),
}

pub struct Compiler {
    ops: Vec<Token>,
}

impl Compiler {
    pub fn new(code: String) -> Result<Compiler, CompilerError> {
        let mut parser = Parser::new(code);
        let ops = match parser.parse() {
            Ok(ops) => ops,
            Err(e) => return Err(CompilerError::ParsingError(e)),
        };
        Ok(Compiler { ops })
    }

    pub fn compile_code(&self) -> Result<String, CompilerError> {
        let mut assembly = String::new();
        let header = indoc! {"
            format ELF64 executable 3

            "};

        let helper_functions = indoc! {"
            ; Helper functions
            SYS_read = 0
            SYS_write = 1
            SYS_exit = 60

            STDIN = 0
            STDOUT = 1

            WRITE_TO_STDOUT:
            mov rax, SYS_write
            mov rdi, STDOUT
            mov rsi, r12
            mov rdx, 1
            syscall
            ret

            READ_FROM_STDIN:
            mov rax, SYS_read
            mov rdi, STDIN
            mov rsi, r12
            mov rdx, 1
            syscall
            ret

            EXIT:
            mov rax, SYS_exit
            mov rdi, 0
            syscall

        "};

        let mut main = indoc! {"
            segment readable executable
            entry main

            main:
            mov r12, (TAPE)
            "}
        .to_string();

        let mut jump_stack = Vec::new();
        for op in self.ops.iter() {
            if op.token_type == TokenType::Eof {
                if !jump_stack.is_empty() {
                    return Err(CompilerError::UnexpectedEof);
                }
                main.push_str(&formatdoc! {"
                        ; TokenType::Eof
                        call EXIT
                    "});
                break;
            }

            let size = match op.size {
                Some(size) => size,
                None => return Err(CompilerError::UnexpectedNoneSize(op.loc)),
            };

            match op.token_type {
                TokenType::Plus => main.push_str(&formatdoc! {"
                        ; TokenType::Plus
                        add byte [r12], {size}
                    "}),
                TokenType::Minus => main.push_str(&formatdoc! {"
                        ; TokenType::Minus
                        sub byte [r12], {size}
                    "}),
                TokenType::ShiftRight => main.push_str(&formatdoc! {"
                        ; TokenType::ShiftRight
                        add r12, {size}
                    "}),
                TokenType::ShiftLeft => main.push_str(&formatdoc! {"
                        ; TokenType::ShiftLeft
                        sub r12, {size}
                    "}),
                TokenType::Dot => {
                    main.push_str("; TokenType::Dot\n");
                    for _ in 0..size {
                        main.push_str("  call WRITE_TO_STDOUT\n");
                    }
                }
                TokenType::Comma => {
                    main.push_str("; TokenType::Comma\n");
                    for _ in 0..size {
                        main.push_str(&formatdoc! {"
                            call READ_FROM_STDIN
                            mov rax, [r12]
                        "});
                    }
                }
                TokenType::OpenBracket => {
                    jump_stack.push(size);
                    let code = formatdoc! {"

                        ; TokenType::OpenBracket
                        cmp byte [r12], 0
                        je after_loop_{size}

                        loop_{size}:

                        "};
                    main.push_str(&code);
                }
                TokenType::CloseBracket => {
                    let loop_name = jump_stack
                        .pop()
                        .ok_or(CompilerError::UnmatchedBracket(op.loc))?;
                    let code = formatdoc! {"

                        ; TokenType::CloseBracket
                        cmp byte [r12], 0
                        jne loop_{loop_name}

                        after_loop_{loop_name}:
                    "};
                    main.push_str(&code);
                }
                TokenType::Eof => {}
            }
        }

        let data = indoc! {"

            segment readable writeable
            TAPE_SIZE = 30000
            TAPE rd TAPE_SIZE
        "};

        assembly.push_str(header);
        assembly.push_str(helper_functions);
        assembly.push_str(&main);
        assembly.push_str(data);

        Ok(assembly)
    }
}
