use crate::execution_context::ExecutionContext;

pub mod general;
pub mod io;
pub mod math;
pub mod string;

pub fn execute_operator(context: &mut ExecutionContext, operator: &String) -> Result<(), String> {
    if math::execute(context, operator)? {
        return Ok(());
    }
    if io::execute(context, operator)? {
        return Ok(());
    }
    if general::execute(context, operator)? {
        return Ok(());
    }
    if string::execute(context, operator)? {
        return Ok(());
    }

    Err(format!("Unknown operator: {}", operator))
}
