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

    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    pub fn peek_offset(&self, offset: isize) -> Option<char> {
        // @NOTE -1 to match caller expectations
        // Because `position` has already been incremented by next()
        //  peek_offset(1)  = peek()    = the next character
        //  peek_offset(2)  =           = the character after next
        //  peek_offset(-1) =           = previous character before current
        let i = (self.position as isize) + offset - 1;
        if i < 0 {
            return None;
        }
        self.chars.get(i as usize).copied()
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
