use crate::interpreter::{Interpreter, Operand};

pub fn execute(interpreter: &mut Interpreter, operator: &str) -> Result<bool, String> {
    match operator {
        "print" => {
            let operand = interpreter.current_scope().pop_operand_any()?;

            // Common print function
            let print_value = |op| match op {
                Operand::Number(value) => println!("{value}"),
                Operand::String(value) => println!("{value}"),
                Operand::Variable(_) => panic!("Cannot print: Invalid type. Variable pointing to a variable."),
            };

            match operand {
                // Variables get dereferenced first, then printed
                Operand::Variable(variable_name) => {
                    if let Some(value) = interpreter.current_scope().get_variable(variable_name) {
                        print_value(value);
                    }
                }
                // Any thing else, print directly
                operand => {
                    print_value(operand);
                }
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}
