use chess_tokenizer::cli::get_file_by_path;
use chess_tokenizer::read_file;
use chess_tokenizer::opening_tree::{GameNode, ViewPerspective};
use chess_tokenizer::opening_tree::build::{build_tree};
use chess_tokenizer::stats::init_stats;


fn main() {
    loop {
        println!("Enter a pgn file to get started");
        // TODO: Maybe structure this more like the mini grep project

        let path = get_file_by_path();

        let formatted_game_matrix = read_file(path).unwrap();

        let mut root  = GameNode::init(init_stats(&formatted_game_matrix), formatted_game_matrix.len());
        // TODO: Change this to something called "depth" and have the game only process the data as far as the current depth
        // of the user interaction
        let max_moves = 3;
        let view_perspective = ViewPerspective::White(String::from("white"));

        build_tree(&mut root, &formatted_game_matrix, &max_moves, &view_perspective);

        // println!("root node is: {:#?}", root);
        // println!("roots children keys are: {:?}", root.get_child_keys());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_of_e4_c5() {
        let formatted_game_matrix = read_file(String::from("games/lichess_EpictetusZ1_2024-01-17.pgn")).unwrap(); // Adjust as needed for testing
        let mut root = GameNode::init(init_stats(&formatted_game_matrix), formatted_game_matrix.len());
        let max_moves = 3;
        let view_perspective = ViewPerspective::White(String::from("white"));

        build_tree(&mut root, &formatted_game_matrix, &max_moves, &view_perspective);

        let e4_node = root.children.get("e4").expect("No 'e4' move found at the root");
        let c5_node = e4_node.children.get("c5").expect("No 'c5' move found after 'e4'");

        assert_eq!(c5_node.frequency, 120, "Frequency for '1. e4 c5' is incorrect");
    }
}
