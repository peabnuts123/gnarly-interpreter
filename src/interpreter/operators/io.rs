use crate::interpreter::Interpreter;

pub fn execute(interpreter: &mut Interpreter, operator: &str) -> Result<bool, String> {
    match operator {
        "print" => {
            let operand = interpreter.current_scope().pop_operand_any()?;
            let output = interpreter.current_scope().operand_to_string(&operand)?;
            println!("{output}");
            Ok(true)
        }
        "print.stack" => {
            print!("Stack [");
            let scope = interpreter.current_scope_readonly();
            let stack = scope.get_operand_stack();
            for (i, operand) in stack.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", scope.operand_display(operand));
            }
            println!("]");
            Ok(true)
        }
        _ => Ok(false),
    }
}
