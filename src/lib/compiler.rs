use crate::lexer::{Token, TokenType};
use crate::parser::{Parser, ParserError};
use indoc::{formatdoc, indoc};
use thiserror::Error;

/// Error type for the compiler
///
/// This error type is used to represent the different kinds of errors that can occur during the
/// compilation
#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Parsing error: {0}")]
    ParsingError(ParserError),
    #[error("Unexpected none size at {0}")]
    UnexpectedNoneSize(usize),
    #[error("Unexpected end of file")]
    UnexpectedEof,
}

/// The settings for the compiler
///
/// This struct is used to represent the settings for the compiler. It contains the wrap setting
/// which is used to determine whether the tape should wrap around or not
/// # Fields
/// * `wrap` - Whether the tape should wrap around or not
/// # Example
/// ```
/// use rbfc::compiler::{CompilerSettings};
/// let settings = CompilerSettings { wrap: true };
/// ```
#[derive(Debug, Default)]
pub struct CompilerSettings {
    pub wrap: bool,
}

/// The compiler struct
///
/// This struct is used to represent the compiler. It contains the operations for the program
///
/// # Fields
/// * `ops` - The operations for the program
///
/// # Example
/// ```
/// use rbfc::compiler::{Compiler, CompilerSettings};
/// let compiler = Compiler::new("+++".to_string(), CompilerSettings::default()).unwrap();
/// ```
#[derive(Debug)]
pub struct Compiler {
    ops: Vec<Token>,
    settings: CompilerSettings,
}

impl Compiler {
    /// Create a new compiler
    ///
    /// This function is used to create a new compiler. It takes a string of code and returns a
    /// Result containing the compiler or a CompilerError
    /// # Arguments
    /// * `code` - The code to compile
    /// # Example
    /// ```
    /// use rbfc::compiler::{Compiler, CompilerSettings};
    /// let compiler = Compiler::new("+++".to_string(), CompilerSettings::default() ).unwrap();
    /// ```
    /// # Errors
    /// If the code cannot be parsed, a CompilerError::ParsingError will be returned
    /// ```
    /// use rbfc::compiler::{Compiler, CompilerError, CompilerSettings};
    /// use rbfc::parser::ParserError;
    ///
    /// matches!(Compiler::new("+++[".to_string(), CompilerSettings::default()), Err(CompilerError::ParsingError(ParserError::UnmatchedBracket(3))));
    /// ```
    pub fn new(code: String, settings: CompilerSettings) -> Result<Compiler, CompilerError> {
        let mut parser = Parser::new(code);
        let ops = match parser.parse() {
            Ok(ops) => ops,
            Err(e) => return Err(CompilerError::ParsingError(e)),
        };
        Ok(Compiler { ops, settings })
    }

    /// Compile the code
    /// This function is used to compile the code. It returns a Result containing the assembly code
    /// or a CompilerError
    /// # Example
    /// ```
    /// use rbfc::compiler::{Compiler, CompilerError, CompilerSettings};
    /// let compiler = Compiler::new("+++".to_string(), CompilerSettings::default()).unwrap();
    /// let asm = compiler.compile_code();
    /// ```
    pub fn compile_code(&self) -> String {
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
                main.push_str(&formatdoc! {"
                        ; TokenType::Eof
                        call EXIT
                    "});
                break;
            }

            let size = match op.size {
                Some(size) => size,
                None => panic!(
                    "Unexpected none size at {}, should be caught at parse",
                    op.loc
                ),
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
                TokenType::ShiftRight => {
                    if self.settings.wrap {
                        main.push_str(&formatdoc! {"
                            ; TokenType::ShiftRight
                            add r12, {size}
                            cmp r12, (TAPE + TAPE_SIZE)
                            jl no_wrap_{loc}
                            sub r12, TAPE_SIZE
                            no_wrap_{loc}:
                        ", loc = op.loc})
                    } else {
                        main.push_str(&formatdoc! {"
                            ; TokenType::ShiftRight
                            add r12, {size}
                        "})
                    }
                }
                TokenType::ShiftLeft => {
                    if self.settings.wrap {
                        main.push_str(&formatdoc! {"
                            ; TokenType::ShiftLeft
                            cmp r12, (TAPE + {size})
                            jl no_wrap_{loc}
                            add r12, TAPE_SIZE
                            sub r12, {size}
                            no_wrap_{loc}:
                        ", loc = op.loc})
                    } else {
                        main.push_str(&formatdoc! {"
                            ; TokenType::ShiftLeft
                            sub r12, {size}
                        "})
                    }
                }
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
                        .expect("Unmatched bracket should be caught at parse");
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

        assembly
    }
}

#[cfg(test)]
mod test {
    use indoc::formatdoc;

    #[test]
    fn compiler_test() {
        use super::{Compiler, CompilerSettings};
        let compiler = Compiler::new("+++".to_string(), CompilerSettings::default()).unwrap();
        let asm = compiler.compile_code();
        assert_eq!(
            asm,
            formatdoc! {
                "format ELF64 executable 3

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
                segment readable executable
                entry main

                main:
                mov r12, (TAPE)
                ; TokenType::Plus
                add byte [r12], 3
                ; TokenType::Eof
                call EXIT

                segment readable writeable
                TAPE_SIZE = 30000
                TAPE rd TAPE_SIZE
            "}
        );
    }
}
