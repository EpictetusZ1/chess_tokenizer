use chess_tokenizer::read_file;
use chess_tokenizer::opening_tree::{GameNode, ViewPerspective};
use chess_tokenizer::opening_tree::build::{build_tree, init_stats};


// TODO: Create input loop, to interact with tree
fn main() {
    // TODO: Have this accept a path as its arg
    let formatted_game_matrix = read_file().unwrap();
    // let mut root  = GameNode::new(0, init_stats(&formatted_game_matrix));
    let mut root  = GameNode::init(init_stats(&formatted_game_matrix), formatted_game_matrix.len());
    let max_moves = 3;
    let view_perspective = ViewPerspective::White(String::from("white"));

    build_tree(&mut root, &formatted_game_matrix, &max_moves, &view_perspective);

    // println!("root node is: {:#?}", root);
    // println!("roots children keys are: {:?}", root.get_child_keys());
    if let Some(e4_node) = root.children.get("e4") {
        if let Some(c5_node) = e4_node.children.get("c5") {
            println!("Frequency for '1. e4 c5': {}", c5_node.frequency);
        } else {
            println!("No 'c5' move found after 'e4'");
        }
    } else {
        println!("No 'e4' move found at the root");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_of_e4_c5() {
        let formatted_game_matrix = read_file().unwrap(); // Adjust as needed for testing
        let mut root = GameNode::init(init_stats(&formatted_game_matrix), formatted_game_matrix.len());
        let max_moves = 3;
        let view_perspective = ViewPerspective::White(String::from("white"));

        build_tree(&mut root, &formatted_game_matrix, &max_moves, &view_perspective);

        let e4_node = root.children.get("e4").expect("No 'e4' move found at the root");
        let c5_node = e4_node.children.get("c5").expect("No 'c5' move found after 'e4'");

        assert_eq!(c5_node.frequency, 120, "Frequency for '1. e4 c5' is incorrect");
    }
}
