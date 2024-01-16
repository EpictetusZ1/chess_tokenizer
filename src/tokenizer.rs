// TODO:
// 1. Normalize input
// 2. Define what a token is
// 3. Define token types
// 4. Define tokenization rules
// 5. Create builder for tokens

// Example Input
// Built Game Data: Game { tags: {"Site": "Linares (Spain)", "Date": "1997.??.??", "Result": "0-1", "Event": "It (cat.19)",
// "White": "Alexei Shirov", "Black": "Garry Kasparov", "Round": "?"}, moves:
// ["e4", "c5", "Nf3", "d6", "d4", "cxd4", "Nxd4", "Nf6", "Nc3", "a6", "Be3", "Ng4", "Bg5", "h6", "Bh4", "g5", "Bg3", "Bg7",
// "Be2", "h5", "Bxg4", "Bxg4", "f3", "Bd7", "O-O", "Nc6", "Bf2", "e6", "Nce2", "Ne5", "b3", "g4", "f4", "h4", "Be3", "h3",
// "g3", "Nc6", "Qd3", "O-O", "Rad1", "f5", "c4", "Qa5", "Nc3", "Rae8", "Rfe1", "e5", "Nxc6", "Bxc6", "b4", "Qa3", "b5", "exf4",
// "Bxf4", "axb5", "cxb5", "Qc5+", "Be3", "Qxc3", "bxc6", "Qxc6", "Qxd6", "Qxe4", "Qd5+", "Qxd5", "Rxd5", "Bc3", "Re2",
// "Re4", "Kf2", "Rfe8", "Rd3", "Bf6", "Red2", "Rxe3"], result: B }

struct Tokenizer {
    input: String,
    tokens: Vec<Token>,
}

enum Token {
    Tag,
    Move,
    Result,
}

// Create Impl block for Sequence checking
impl Tokenizer {
    fn new(input: String) -> Tokenizer {
        Tokenizer {
            input,
            tokens: Vec::new(),
        }
    }

    fn tokenize(&mut self) {}
}
