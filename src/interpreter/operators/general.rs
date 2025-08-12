use crate::interpreter::Interpreter;

pub fn execute(interpreter: &mut Interpreter, operator: &str) -> Result<bool, String> {
    match operator {
        "set" => {
            let variable_name = interpreter
                .current_scope()
                .pop_operand_variable_identifier()?;
            let value = interpreter.current_scope().pop_operand_any()?;
            interpreter
                .current_scope()
                .set_variable(variable_name, value);
            Ok(true)
        }
        _ => Ok(false),
    }
}
