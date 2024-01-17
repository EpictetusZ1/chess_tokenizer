use chess_tokenizer::read_file;
// use chess_tokenizer::opening_book;


fn main() {
    let formatted_game_matrix = read_file().unwrap();

    // println!("Formatted Game Matrix: {:?}", formatted_game_matrix);

    for m in formatted_game_matrix.iter() {
        for p in &m.moves {
            println!("p is: {:?}", p);
        }
    }

    // let tree = opening_book::GameNode::new()
}
