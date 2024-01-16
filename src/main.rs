use chess_tokenizer::read_file;

fn main() {
    if let Err(e) = read_file() {
        eprintln!("Error reading file: {}", e);
    }
}
