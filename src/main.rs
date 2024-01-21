use opening_tree::cli::{get_file_by_path};
use opening_tree::{cli, read_file};
use opening_tree::opening_tree::{GameNode, ViewPerspective};
use opening_tree::opening_tree::build::{build_tree};
use opening_tree::opening_tree::navigator::GameTreeNavigator;
use opening_tree::stats::init_stats;



fn main() {
        println!("Enter a pgn file to get started: ");

        let path = get_file_by_path();
        let formatted_game_matrix = read_file(path).unwrap();
        let max_moves = 3;

        let mut root = GameNode::init(init_stats(&formatted_game_matrix), formatted_game_matrix.len());
        let view_perspective = ViewPerspective::White(String::from("white"));

        build_tree(&mut root, &formatted_game_matrix, &max_moves, &view_perspective);

        let mut navigator = GameTreeNavigator::new(root);

        cli::run_cli(&mut navigator);
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
