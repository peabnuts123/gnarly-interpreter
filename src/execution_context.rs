use crate::{execution_context::scope::Scope, interpreter::Operand};

pub mod scope;

pub struct ExecutionContext {
    scopes: Vec<Scope>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }

    pub fn push_new_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) -> Scope {
        if self.scopes.len() == 1 {
            panic!("Invalid operation: Cannot pop root scope")
        }

        self.scopes
            .pop()
            .expect("Unexpected error: No scopes to pop")
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

    /// Convert an operand into a string, for printing.
    /// e.g. `"hello"` => `hello`
    /// e.g. `$name` => `Michael`
    pub fn operand_to_string(&self, operand: &Operand) -> Result<String, String> {
        match operand {
            Operand::Number(value) => Ok(value.to_string()),
            Operand::String(value) => Ok(value.clone()),
            Operand::Variable(name) => match self.get_variable(name) {
                Some(inner) => self.operand_to_string(&inner),
                None => Err(format!("Cannot stringify: Variable '{}' not found", name)),
            },
            Operand::Scope(scope) => {
                let mut parts = Vec::new();

                // Add operands as array elements
                let operands = scope.get_operand_stack();
                for operand in operands {
                    parts.push(self.operand_display(operand));
                }

                // Add variables as key-value pairs
                let variables = scope.get_variable_state();
                for (name, value) in variables {
                    parts.push(format!("{} = {}", name, self.operand_display(value)));
                }

                Ok(format!("{{ {} }}", parts.join(", ")))
            }
        }
    }

    /// Display the actual runtime value of an operand, for debugging.
    /// e.g. `"hello"` => `"hello"`,
    /// e.g. `$name` => `$name ("Michael")`
    pub fn operand_display(&self, operand: &Operand) -> String {
        match operand {
            Operand::Number(value) => format!("{}", value),
            Operand::String(value) => format!("\"{}\"", value),
            // Operand::Variable(name) => println!("${}", name),
            Operand::Variable(name) => match self.get_variable(name) {
                Some(inner) => format!("${} ({})", name, self.operand_display(&inner)),
                None => format!("${} (unset)", name),
            },
            Operand::Scope(scope) => {
                format!("Scope({})", scope.get_operand_stack().len() + scope.get_variable_state().len())
            }
        }
    }

    pub fn interpolate_string_variables(&self, input: &str) -> Result<String, String> {
        // @TODO Share regex with lexer code
        let re = regex::Regex::new(r"\$([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
        let mut result = String::new();
        let mut last_end = 0;
        for cap in re.captures_iter(input) {
            if let Some(m) = cap.get(0) {
                result.push_str(&input[last_end..m.start()]);
                let var_name = &cap[1];
                if let Some(val) = self.get_variable(&var_name.to_string()) {
                    result.push_str(&self.operand_to_string(&val)?);
                } else {
                    return Err(format!(
                        "Cannot interpolate: Variable '{}' not found",
                        var_name
                    ));
                }
                last_end = m.end();
            }
        }
        result.push_str(&input[last_end..]);
        Ok(result)
    }

    pub fn set_variable(&mut self, name: String, value: Operand) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.has_variable(name.clone()) {
                scope.set_variable(name, value);
                return;
            }
        }

        // If no scope has the variable, create a new one in the current scope
        self.current_scope().set_variable(name, value);
    }

    pub fn get_variable(&self, name: &String) -> Option<&Operand> {
        for scope in self.scopes.iter().rev() {
            if scope.has_variable(name.clone()) {
                return scope.get_variable(name);
            }
        }
        None
    }

    pub fn push_operand(&mut self, operand: Operand) {
        self.current_scope().push_operand(operand);
    }

    pub fn pop_operand_number_literal(&mut self) -> Result<f64, String> {
        self._pop_operand_and_parse("NumberLiteral", true, |token| match token {
            Operand::Number(value) => Some(value),
            _ => None,
        })
    }

    pub fn pop_operand_string_literal(&mut self) -> Result<String, String> {
        self._pop_operand_and_parse("StringLiteral", true, |token| match token {
            Operand::String(value) => Some(value),
            _ => None,
        })
    }

    pub fn pop_operand_variable_identifier(&mut self) -> Result<String, String> {
        self._pop_operand_and_parse("VariableIdentifier", false, |token| match token {
            Operand::Variable(value) => Some(value),
            _ => None,
        })
    }

    pub fn pop_operand_any(&mut self) -> Result<Operand, String> {
        let token = self.current_scope().pop_operand();
        match token {
            Some(token) => Ok(token),
            None => Err(format!("Token stack is empty!")),
        }
    }

    fn _pop_operand_and_parse<TResult, F>(
        &mut self,
        token_type_name: &str,
        lookup_variable_value: bool,
        parser: F,
    ) -> Result<TResult, String>
    where
        TResult: Clone,
        F: FnOnce(&Operand) -> Option<&TResult>,
    {
        let popped_operand = self.current_scope().pop_operand();
        let mut operand = popped_operand.as_ref();

        if lookup_variable_value && let Some(Operand::Variable(variable_name)) = operand {
            operand = self.get_variable(variable_name);
        }

        match operand {
            Some(token) => {
                // @NOTE Construct error message proactively, to prevent having to .clone() `token`
                let maybe_err_message = format!(
                    "Expected token of type '{}' but found: {:?}",
                    token_type_name, token
                );
                match parser(token) {
                    Some(result) => Ok(result.clone()),
                    None => Err(maybe_err_message),
                }
            }
            None => Err(format!("Token stack is empty!")),
        }
    }
}
