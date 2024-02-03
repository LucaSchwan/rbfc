use rbfc::interpreter::{Interpreter, InterpreterError};

mod rbfc;

fn main() -> Result<(), InterpreterError> {
    let input = String::from("+++.>+++.>,.>,.");
    let mut interpreter = Interpreter::new(input, vec![3, 3])?;
    interpreter.interpret().unwrap();
    Ok(())
}
