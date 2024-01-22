use opening_tree::cli::{get_file_by_path};
use opening_tree::{cli, read_file};
use opening_tree::opening_tree::{GameNode, ViewPerspective};
use opening_tree::opening_tree::navigator::GameTreeNavigator;
use opening_tree::stats::init_stats;


fn main() {
        println!("Enter a pgn file to get started: ");

        let path = get_file_by_path();
        let formatted_game_matrix = read_file(path).unwrap();

        let mut root = GameNode::init(init_stats(&formatted_game_matrix), formatted_game_matrix.len());
        let view_perspective = ViewPerspective::White(String::from("white"));

        GameTreeNavigator::build_tree(&mut root, &formatted_game_matrix, &view_perspective);

        let mut navigator = GameTreeNavigator::new(root);

        cli::run_cli(&mut navigator, &formatted_game_matrix, &view_perspective);
}
