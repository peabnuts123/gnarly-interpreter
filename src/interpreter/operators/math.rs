use crate::{execution_context::ExecutionContext, interpreter::Operand};

pub fn execute(context: &mut ExecutionContext, operator: &str) -> Result<bool, String> {
    match operator {
        "+" => {
            let right = context.pop_operand_number_literal()?;
            let left = context.pop_operand_number_literal()?;
            context.push_operand(Operand::Number(left + right));
            Ok(true)
        }
        "-" => {
            let right = context.pop_operand_number_literal()?;
            let left = context.pop_operand_number_literal()?;
            context.push_operand(Operand::Number(left - right));
            Ok(true)
        }
        "*" => {
            let right = context.pop_operand_number_literal()?;
            let left = context.pop_operand_number_literal()?;
            context.push_operand(Operand::Number(left * right));
            Ok(true)
        }
        "/" => {
            let right = context.pop_operand_number_literal()?;
            let left = context.pop_operand_number_literal()?;
            match right {
                0.0 => Err(format!("Division by zero")),
                _ => {
                    context.push_operand(Operand::Number(left / right));
                    Ok(true)
                }
            }
        }
        _ => Ok(false), // This module doesn't handle this operator
    }
}
