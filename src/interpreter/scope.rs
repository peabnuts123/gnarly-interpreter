use std::collections::HashMap;

use crate::interpreter::Operand;

pub struct Scope {
    operand_stack: Vec<Operand>,
    variable_state: HashMap<String, Operand>,
}

impl Scope {
    pub fn new() -> Scope {
        Self {
            operand_stack: Vec::new(),
            variable_state: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: Operand) {
        if let Operand::Variable(_) = value {
            // @TODO ideally the compiler would enforce this, but we are just
            // kinda re-using the `Operand` type :think:
            panic!("Cannot set variable of type variable");
        }

        self.variable_state.insert(name, value);
    }

    pub fn get_variable(&self, name: String) -> Option<&Operand> {
        self.variable_state.get(&name)
    }

    pub fn push_operand(&mut self, operand: Operand) {
        self.operand_stack.push(operand);
    }

    pub fn pop_operand_number_literal(&mut self) -> Result<f64, String> {
        // @TODO Interpret the value of variables
        self._pop_operand_and_parse("NumberLiteral", |token| match token {
            Operand::Number(value) => Some(value),
            _ => None,
        })
    }
    pub fn pop_operand_variable_identifier(&mut self) -> Result<String, String> {
        self._pop_operand_and_parse("VariableIdentifier", |token| match token {
            Operand::Variable(value) => Some(value),
            _ => None,
        })
    }
    pub fn pop_operand_any(&mut self) -> Result<Operand, String> {
        let token = self.operand_stack.pop();
        match token {
            Some(token) => Ok(token),
            None => Err(format!("Token stack is empty!")),
        }
    }

    fn _pop_operand_and_parse<TResult, F>(
        &mut self,
        token_type_name: &str,
        parser: F,
    ) -> Result<TResult, String>
    where
        F: FnOnce(Operand) -> Option<TResult>,
    {
        match self.operand_stack.pop() {
            Some(token) => match parser(token.clone()) {
                Some(result) => Ok(result),
                None => Err(format!(
                    "Expected token of type '{}' but found: {:?}",
                    token_type_name, token
                )),
            },
            None => Err(format!("Token stack is empty!")),
        }
    }
}