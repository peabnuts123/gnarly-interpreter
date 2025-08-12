use crate::interpreter::{Interpreter, Operand};

pub fn execute(interpreter: &mut Interpreter, operator: &str) -> Result<bool, String> {
    match operator {
        "print" => {
            let operand = interpreter.pop_operand_any()?;
            match operand {
              Operand::Number(value) => println!("{value}")
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}