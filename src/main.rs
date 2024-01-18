use chess_tokenizer::read_file;
// use chess_tokenizer::opening_book;
use chess_tokenizer::opening_tree::{GameNode};
use chess_tokenizer::opening_tree::traverse::traverse_tree;



fn main() {
    let formatted_game_matrix = read_file().unwrap();
    let mut root  = GameNode::new();
    let max_moves = 1;

    traverse_tree(&mut root, formatted_game_matrix, max_moves);
    // println!("root node is: {:?}", root);
    println!("roots children keys are: {:?}", root.get_child_keys());
}


mod tests {
    use super::*;

    #[test]
    fn test_correct_first_ply() {
        let formatted_game_matrix = read_file().unwrap();
        let mut root = GameNode::new();
        let max_moves = 1;

        traverse_tree(&mut root, formatted_game_matrix, max_moves);

        let expected_moves = ["d3", "d4", "e4", "Nf3", "e3", "f4", "g4", "b3", "c4", "Nc3", "g3"];
        let root_child_keys = root.get_child_keys();

        for mov in &expected_moves {
            assert!(root_child_keys.contains(&mov.to_string()), "Move {} not found in root children", mov);
        }
    }
}
