use crate::{interpreter::scope::Scope, lexer::Token};

mod operators;
mod scope;

#[derive(Debug, Clone)]
pub enum Operand {
    Number(f64),
    String(String),
    Variable(String),
}

pub struct Interpreter {
    scopes: Vec<Scope>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }

    pub fn run(&mut self, mut token_stack: Vec<Token>) -> Result<(), String> {
        while token_stack.len() > 0 {
            let token = token_stack.remove(0);
            match token {
                Token::Operator(op) => {
                    operators::execute_operator(self, &op)?
                }
                Token::NumberLiteral(value) => {
                    self.current_scope().push_operand(Operand::Number(value));
                }
                Token::StringLiteral(value) => {
                    let interpolated = self.current_scope().interpolate_string_variables(&value)?;
                    self.current_scope().push_operand(Operand::String(interpolated));
                }
                Token::VariableIdentifier(variable_name) => {
                    self.current_scope().push_operand(Operand::Variable(variable_name));
                }
            }
        }
        Ok(())
    }

    pub fn push_new_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) -> Scope {
        if self.scopes.len() == 1 {
            panic!("Invalid operation: Cannot pop root scope")
        }

        self.scopes.pop().expect("Unexpected error: No scopes to pop")
    }

    pub fn current_scope(&mut self) -> &mut Scope {
        match self.scopes.last_mut() {
            Some(scope) => scope,
            None => panic!("Unexpected error: No execution scopes available"),
        }
    }

    pub fn current_scope_readonly(&self) -> &Scope {
        match self.scopes.last() {
            Some(scope) => scope,
            None => panic!("Unexpected error: No execution scopes available"),
        }
    }
}
