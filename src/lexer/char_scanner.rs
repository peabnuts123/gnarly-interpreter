pub struct CharScanner {
    chars: Vec<char>,
    position: usize,
}

impl CharScanner {
    /// Create a new scanner from input text
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            position: 0,
        }
    }

    /// Get the current character without advancing
    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    /// Get the character at a relative offset from current position
    pub fn peek_ahead(&self, offset: usize) -> Option<char> {
        self.chars.get(self.position + offset).copied()
    }
}

impl Iterator for CharScanner {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.chars.len() {
            let ch = self.chars[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }
}