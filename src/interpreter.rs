use crate::lexer::Token;

mod operators;


pub struct Interpreter {
    token_stack: Vec<Token>,
    pub operand_stack: Vec<Token>,
}

impl Interpreter {
    pub fn new(token_stack: Vec<Token>) -> Self {
        Self {
            token_stack,
            operand_stack: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        while self.token_stack.len() > 0 {
            let token = self.token_stack.remove(0);
            match token {
                Token::NumberLiteral(_) => {
                    self.operand_stack.push(token);
                }
                Token::Operator(op) => {
                    match self.execute_operator(&op) {
                        Ok(_) => {
                            /* 😎 */
                        }
                        Err(err) => {
                            // @TODO line/column number or whatever.
                            eprintln!("Error executing operator '{}': {}", op, err);
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn _pop_operand_and_parse<TResult, F>(
        &mut self,
        token_type_name: &str,
        parser: F,
    ) -> Result<TResult, String>
    where
        F: FnOnce(Token) -> Option<TResult>,
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

    pub fn pop_operand_number_literal(&mut self) -> Result<f64, String> {
        self._pop_operand_and_parse("NumberLiteral", |token| {
            match token {
                Token::NumberLiteral(value) => Some(value),
                _ => None,
            }
        })
    }

    fn execute_operator(&mut self, operator: &String) -> Result<(), String> {
        if operators::math::execute(self, operator)? {
            return Ok(());
        }

        Err(format!("Unknown operator: {}", operator))
    }
}
