use crate::execution_context::ExecutionContext;

pub fn execute(context: &mut ExecutionContext, operator: &str) -> Result<bool, String> {
    match operator {
        "print" => {
            let operand = context.pop_operand_any()?;
            let output = context.operand_to_string(&operand)?;
            println!("{output}");
            Ok(true)
        }
        "print.stack" => {
            print!("Stack [");
            let scope = context.current_scope_readonly();
            let stack = scope.get_operand_stack();
            for (i, operand) in stack.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", context.operand_display(operand));
            }
            println!("]");
            Ok(true)
        }
        _ => Ok(false),
    }
}
