use crate::interpreter::Interpreter;

pub fn execute(interpreter: &mut Interpreter, operator: &str) -> Result<bool, String> {
    match operator {
        "print" => {
            let operand = interpreter.current_scope().pop_operand_any()?;
            let output = interpreter.current_scope().stringify_operand(&operand);
            println!("{output}");
            Ok(true)
        }
        _ => Ok(false),
    }
}
