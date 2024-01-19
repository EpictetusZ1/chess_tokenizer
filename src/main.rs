use chess_tokenizer::read_file;
use chess_tokenizer::opening_tree::{GameNode, ViewPerspective};
use chess_tokenizer::opening_tree::build::build_tree;


// TODO: Create input loop, to interact with tree
fn main() {
    let formatted_game_matrix = read_file().unwrap();
    let mut root  = GameNode::new(0);
    let max_moves = 3;
    let view_perspective = ViewPerspective::White(String::from("white"));

    build_tree(&mut root, &formatted_game_matrix, &max_moves, &view_perspective);

    println!("root node is: {:#?}", root);
    println!("roots children keys are: {:?}", root.get_child_keys());
}

