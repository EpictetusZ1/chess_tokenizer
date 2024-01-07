
// TODO:
// 1. Normalize input
// 2. Define what a token is
// 3. Define token types
// 4. Define tokenization rules
// 5. Create builder for tokens

struct Tokenizer {
    input: String,
    tokens: Vec<Token>,
}

enum Token {
    Tag,
    Move,
    Result,
}

pub fn tokenize() {

}
