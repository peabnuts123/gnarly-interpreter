use crate::lexer::char_scanner::CharScanner;

mod char_scanner;

// Special single-character operators
const SPECIAL_OPERATOR_CHARS: &[char] = &['+', '-', '*', '/'];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexerState {
    Default,
    // Array,
    // Comment,
    NumberLiteral,
    // StringLiteral,
    // BooleanLiteral,
    // NullLiteral,
    Operator,
    // Variable,
    // Word,
}

#[derive(Debug, Clone)]
pub enum Token {
    NumberLiteral(f64),
    Operator(String),
}

impl Token {
    pub fn new_number_literal(raw_value: &str) -> Result<Self, String> {
        // Parse the raw string value into a number literal
        let value = raw_value
            .parse::<f64>()
            .map_err(|_| format!("Invalid number: {}", raw_value))?;
        Ok(Token::NumberLiteral(value))
    }

    pub fn new_operator(raw_value: &str) -> Result<Self, String> {
        // Create an operator token from the raw string value
        if raw_value.is_empty() {
            Err(format!("Cannot create operator token from empty string"))
        } else {
            Ok(Token::Operator(raw_value.to_string()))
        }
    }
}

pub struct Lexer {
    scanner: CharScanner,
    state: LexerState,
    pub token_list: Vec<Token>,
    current_token_bytes: String,
}

enum EvaluateCharResult {
    Valid,
    Invalid(String),
}
enum EndTokenResult {
    Valid,
    Invalid(String),
}

impl Lexer {
    pub fn scan(source_code: &str) -> Result<Lexer, String> {
        // Create lexer (but do not return reference)
        let mut lexer = Self {
            scanner: CharScanner::new(source_code),
            state: LexerState::Default,
            token_list: Vec::new(),
            current_token_bytes: String::new(),
        };

        // Scan source code one character at a time
        // Fail if there is any error
        while let Some(ch) = lexer.scanner.next() {
            if let EvaluateCharResult::Invalid(err) = lexer.evaluate_char(ch) {
                return Err(err);
            }
        }

        // Finalize any remaining token
        lexer.end_token();

        Ok(lexer)
    }

    fn reevaluate_char_in_new_state(&mut self, state: LexerState, ch: char) -> EvaluateCharResult {
        self.state = state;
        self.evaluate_char(ch)
    }

    fn evaluate_char(&mut self, ch: char) -> EvaluateCharResult {
        match self.state {
            LexerState::Default => {
                if ch.is_whitespace() {
                    /* Whitespace */
                    EvaluateCharResult::Valid
                } else if ch.is_digit(10)
                    || (ch == '-' && self.scanner.peek().map_or(false, |c| c.is_digit(10)))
                {
                    /* NumberLiteral */
                    self.current_token_bytes = ch.to_string();
                    self.state = LexerState::NumberLiteral;
                    EvaluateCharResult::Valid
                } else if SPECIAL_OPERATOR_CHARS.contains(&ch) {
                    /* Special symbol operator */
                    self.current_token_bytes = ch.to_string();
                    self.state = LexerState::Operator;
                    EvaluateCharResult::Valid
                } else if ch.is_alphabetic() {
                    /* Word-based operator */
                    self.current_token_bytes = ch.to_string();
                    self.state = LexerState::Operator;
                    EvaluateCharResult::Valid
                } else {
                    /* Unhandled */
                    EvaluateCharResult::Invalid(format!("Unexpected character: '{}'", ch))
                }
            }
            LexerState::NumberLiteral => {
                if ch.is_digit(10) || ch == '.' {
                    // Continue building number literal
                    self.current_token_bytes.push(ch);
                    EvaluateCharResult::Valid
                } else {
                    // Literally any other character ends the number literal
                    // Finalize number literal
                    match self.end_token() {
                        EndTokenResult::Valid => {
                            self.reevaluate_char_in_new_state(LexerState::Default, ch)
                        }
                        EndTokenResult::Invalid(err) => EvaluateCharResult::Invalid(err),
                    }
                }
            }
            LexerState::Operator => {
                // Check if we're building a special symbol operator or word-based operator
                if self.current_token_bytes.len() == 1
                    && SPECIAL_OPERATOR_CHARS.contains(&self.current_token_bytes.chars().next().unwrap()) {
                    // Special symbol operator - end immediately
                    match self.end_token() {
                        EndTokenResult::Valid => {
                            self.reevaluate_char_in_new_state(LexerState::Default, ch)
                        }
                        EndTokenResult::Invalid(err) => EvaluateCharResult::Invalid(err),
                    }
                } else if ch.is_alphanumeric() || ch == '.' {
                    // Continue building word-based operator
                    self.current_token_bytes.push(ch);
                    EvaluateCharResult::Valid
                } else {
                    // End of word-based operator
                    match self.end_token() {
                        EndTokenResult::Valid => {
                            self.reevaluate_char_in_new_state(LexerState::Default, ch)
                        }
                        EndTokenResult::Invalid(err) => EvaluateCharResult::Invalid(err),
                    }
                }
            }
        }
    }

    fn end_token(&mut self) -> EndTokenResult {
        match self.state {
            LexerState::Default => {
                /* No-op */
                EndTokenResult::Valid
            }
            LexerState::NumberLiteral => {
                self.process_new_token(Token::new_number_literal(&self.current_token_bytes))
            }
            LexerState::Operator => {
                self.process_new_token(Token::new_operator(&self.current_token_bytes))
            }
        }
    }
    fn process_new_token(&mut self, token: Result<Token, String>) -> EndTokenResult {
        match token {
            Ok(token) => {
                self.token_list.push(token);
                self.current_token_bytes.clear();
                EndTokenResult::Valid
            }
            Err(err) => EndTokenResult::Invalid(err),
        }
    }
}
