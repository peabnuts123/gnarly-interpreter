use crate::execution_context::ExecutionContext;

pub fn execute(context: &mut ExecutionContext, operator: &str) -> Result<bool, String> {
    match operator {
        "set" => {
            let variable_name = context.pop_operand_variable_identifier()?;
            let value = context.pop_operand_any()?;
            context.set_variable(variable_name, value);
            Ok(true)
        }
        _ => Ok(false),
    }
}
