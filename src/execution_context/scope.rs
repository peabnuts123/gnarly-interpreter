use std::collections::HashMap;

use crate::interpreter::Operand;


#[derive(Debug)]
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

    pub fn has_variable(&self, name: String) -> bool {
        self.variable_state.contains_key(&name)
    }

    pub fn get_variable(&self, name: &String) -> Option<&Operand> {
        self.variable_state.get(name)
    }

    pub fn set_variable(&mut self, name: String, value: Operand) {
        if let Operand::Variable(_) = value {
            // @TODO ideally the compiler would enforce this, but we are just
            // kinda re-using the `Operand` type :think:
            panic!("Cannot set variable of type variable");
        }

        self.variable_state.insert(name, value);
    }

    pub fn pop_operand(&mut self) -> Option<Operand> {
        self.operand_stack.pop()
    }

    pub fn push_operand(&mut self, operand: Operand) {
        self.operand_stack.push(operand);
    }

    pub fn get_operand_stack(&self) -> &Vec<Operand> {
        &self.operand_stack
    }
    pub fn get_variable_state(&self) -> &HashMap<String, Operand> {
        &self.variable_state
    }


}
