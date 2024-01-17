use crate::GameResult;

#[derive(Debug)]
pub struct Tokenizer {
    input: String,
    tokens: Vec<String>,
}

// This is not currently used, but I might use it later
#[derive(Debug)]
pub(crate) enum Token {
    Tag(String, String),
    Result(GameResult),
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        Tokenizer {
            input,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) {
        let mut current_move = String::new();
        let mut is_reading_move = false;

        for ch in self.input.chars() {
            if ch.is_whitespace() {
                if !current_move.is_empty() {
                    self.tokens.push(current_move.clone());
                    current_move.clear();
                }
                is_reading_move = false;
            } else if ch.is_alphanumeric() || ch == 'O' || ch == '-' || ch == '+' || ch == '#' || ch == 'x' {
                if !is_reading_move {
                    is_reading_move = true;
                }
                current_move.push(ch);
            }
        }

        // Add last move if not ending in space
        if !current_move.is_empty() {
            self.tokens.push(current_move);
        }
    }
}
