use crate::interpreter::{Interpreter, Operand};

pub fn execute(interpreter: &mut Interpreter, operator: &str) -> Result<bool, String> {
    match operator {
        "+" => {
            let right = interpreter.pop_operand_number_literal()?;
            let left = interpreter.pop_operand_number_literal()?;
            interpreter.operand_stack.push(Operand::Number(left + right));
            Ok(true)
        }
        "-" => {
            let right = interpreter.pop_operand_number_literal()?;
            let left = interpreter.pop_operand_number_literal()?;
            interpreter.operand_stack.push(Operand::Number(left - right));
            Ok(true)
        }
        "*" => {
            let right = interpreter.pop_operand_number_literal()?;
            let left = interpreter.pop_operand_number_literal()?;
            interpreter.operand_stack.push(Operand::Number(left * right));
            Ok(true)
        }
        "/" => {
            let right = interpreter.pop_operand_number_literal()?;
            let left = interpreter.pop_operand_number_literal()?;
            match right {
                0.0 => Err(format!("Division by zero")),
                _ => {
                    interpreter.operand_stack.push(Operand::Number(left / right));
                    Ok(true)
                }
            }
        }
        _ => Ok(false), // This module doesn't handle this operator
    }
}