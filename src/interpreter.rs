use crate::{interpreter::scope::Scope, lexer::Token};

mod operators;
mod scope;

#[derive(Debug, Clone)]
pub enum Operand {
    Number(f64),
    Variable(String),
}

pub struct Interpreter {
    token_stack: Vec<Token>,
    scopes: Vec<Scope>,
}

impl Interpreter {
    pub fn new(token_stack: Vec<Token>) -> Self {
        Self {
            token_stack,
            scopes: vec![Scope::new()],
        }
    }

    pub fn run(&mut self) {
        while self.token_stack.len() > 0 {
            let token = self.token_stack.remove(0);
            match token {
                Token::Operator(op) => {
                    match operators::execute_operator(self, &op) {
                        Ok(_) => { /* 😎 */ }
                        Err(err) => {
                            // @TODO line/column number or whatever.
                            eprintln!("Error executing operator '{}': {}", op, err);
                            return;
                        }
                    }
                }
                Token::NumberLiteral(value) => {
                    self.current_scope().push_operand(Operand::Number(value));
                }
                Token::VariableIdentifier(variable_name) => {
                    self.current_scope().push_operand(Operand::Variable(variable_name));
                }
            }
        }
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
}
