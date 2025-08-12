use crate::interpreter::Interpreter;

pub mod general;
pub mod io;
pub mod math;

pub fn execute_operator(interpreter: &mut Interpreter, operator: &String) -> Result<(), String> {
    if math::execute(interpreter, operator)? {
        return Ok(());
    }
    if io::execute(interpreter, operator)? {
        return Ok(());
    }
    if general::execute(interpreter, operator)? {
        return Ok(());
    }

    Err(format!("Unknown operator: {}", operator))
}
