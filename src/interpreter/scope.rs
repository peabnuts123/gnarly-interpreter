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

    pub fn stringify_operand(&self, operand: &Operand) -> Result<String, String> {
        match operand {
            Operand::Number(value) => Ok(value.to_string()),
            Operand::String(value) => Ok(value.clone()),
            Operand::Variable(name) => {
                match self.get_variable(name.clone()) {
                    Some(inner) => self.stringify_operand(&inner),
                    None => Err(format!("Cannot stringify: Variable '{}' not found", name)),
                }
            }
        }
    }

    pub fn interpolate_string_variables(&self, input: &str) -> Result<String, String> {
        // @TODO Share with lexer code
        let re = regex::Regex::new(r"\$([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
        let mut result = String::new();
        let mut last_end = 0;
        for cap in re.captures_iter(input) {
            if let Some(m) = cap.get(0) {
                result.push_str(&input[last_end..m.start()]);
                let var_name = &cap[1];
                if let Some(val) = self.get_variable(var_name.to_string()) {
                    result.push_str(&self.stringify_operand(&val)?);
                } else {
                    return Err(format!("Cannot interpolate: Variable '{}' not found", var_name));
                }
                last_end = m.end();
            }
        }
        result.push_str(&input[last_end..]);
        Ok(result)
    }

    pub fn set_variable(&mut self, name: String, value: Operand) {
        if let Operand::Variable(_) = value {
            // @TODO ideally the compiler would enforce this, but we are just
            // kinda re-using the `Operand` type :think:
            panic!("Cannot set variable of type variable");
        }

        self.variable_state.insert(name, value);
    }

    pub fn get_variable(&self, name: String) -> Option<Operand> {
        self.variable_state.get(&name).cloned()
    }

    pub fn push_operand(&mut self, operand: Operand) {
        self.operand_stack.push(operand);
    }

    pub fn pop_operand_number_literal(&mut self) -> Result<f64, String> {
        self._pop_operand_and_parse("NumberLiteral", |token| match token {
            Operand::Number(value) => Some(value),
            _ => None,
        }, true)
    }
    pub fn pop_operand_string_literal(&mut self) -> Result<String, String> {
        self._pop_operand_and_parse("StringLiteral", |token| match token {
            Operand::String(value) => Some(value),
            _ => None,
        }, true)
    }
    pub fn pop_operand_variable_identifier(&mut self) -> Result<String, String> {
        self._pop_operand_and_parse("VariableIdentifier", |token| match token {
            Operand::Variable(value) => Some(value),
            _ => None,
        }, false)
    }
    pub fn pop_operand_any(&mut self) -> Result<Operand, String> {
        let token = self.operand_stack.pop();
        match token {
            Some(token) => Ok(token),
            None => Err(format!("Token stack is empty!")),
        }
    }

    pub fn get_operand_stack(&self) -> &Vec<Operand> {
        &self.operand_stack
    }

    fn _pop_operand_and_parse<TResult, F>(
        &mut self,
        token_type_name: &str,
        parser: F,
        lookup_variable_value: bool,
    ) -> Result<TResult, String>
    where
        F: FnOnce(Operand) -> Option<TResult>,
    {
        let mut operand = self.operand_stack.pop();

        if lookup_variable_value && let Some(Operand::Variable(variable_name)) = operand {
            operand = self.get_variable(variable_name);
        }

        match operand {
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