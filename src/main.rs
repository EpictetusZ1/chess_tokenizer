use chess_tokenizer::read_file;
// use chess_tokenizer::opening_book;
use chess_tokenizer::opening_book::{GameNode};
use chess_tokenizer::opening_book::traverse::traverse_tree;



fn main() {
    let formatted_game_matrix = read_file().unwrap();
    let mut root  = GameNode::new();
    let max_moves = 1;

    traverse_tree(&mut root, formatted_game_matrix, max_moves);
    // println!("root node is: {:?}", root);
    println!("roots children keys are: {:?}", root.get_child_keys());
}

