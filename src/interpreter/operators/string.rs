use crate::{execution_context::ExecutionContext, interpreter::Operand};

pub fn execute(context: &mut ExecutionContext, operator: &str) -> Result<bool, String> {
    match operator {
        "string.concat" => {
            let right = context.pop_operand_string_literal()?;
            let left = context.pop_operand_string_literal()?;
            context.push_operand(Operand::String(format!("{}{}", left, right)));
            Ok(true)
        }
        _ => Ok(false),
    }
}
