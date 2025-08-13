use crate::interpreter::{Interpreter, Operand};

pub fn execute(interpreter: &mut Interpreter, operator: &str) -> Result<bool, String> {
    match operator {
        "string.concat" => {
            let right = interpreter.current_scope().pop_operand_string_literal()?;
            let left = interpreter.current_scope().pop_operand_string_literal()?;
            interpreter.current_scope().push_operand(Operand::String(format!("{}{}", left, right)));
            Ok(true)
        }
        _ => Ok(false),
    }
}
