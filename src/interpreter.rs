use crate::{execution_context::{scope::Scope, ExecutionContext}, lexer::Token};

mod operators;

#[derive(Debug)]
pub enum Operand {
    Number(f64),
    String(String),
    Variable(String),
    Scope(Scope),
}

pub struct Interpreter {
    pub context: ExecutionContext,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            context: ExecutionContext::new(),
        }
    }

    pub fn run(&mut self, mut token_stack: Vec<Token>) -> Result<(), String> {
        while token_stack.len() > 0 {
            let token = token_stack.remove(0);
            match token {
                Token::Operator(op) => operators::execute_operator(&mut self.context, &op)?,
                Token::NumberLiteral(value) => {
                    self.context.push_operand(Operand::Number(value));
                }
                Token::StringLiteral(value) => {
                    let interpolated = self.context.interpolate_string_variables(&value)?;
                    self.context.push_operand(Operand::String(interpolated));
                }
                Token::VariableIdentifier(variable_name) => {
                    self.context.push_operand(Operand::Variable(variable_name));
                }
                Token::ScopeStart => {
                    self.context.push_new_scope();
                }
                Token::ScopeEnd => {
                    let scope = self.context.pop_scope();
                    self.context.push_operand(Operand::Scope(scope));
                }
            }
        }
        Ok(())
    }




}
