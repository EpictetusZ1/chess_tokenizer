#![allow(unused_imports)]
use std::env;
use opening_tree::cli::{get_file_by_path};
use opening_tree::{cli, Game, read_file};
use opening_tree::opening_tree::{GameNode, ViewPerspective};
use opening_tree::opening_tree::navigator::GameTreeNavigator;
use opening_tree::opening_tree::test::{build_subtree_tree_for_game, ChessMove, OpeningBook};
use opening_tree::stats::init_stats;

fn display_moves_one_step_ahead(node: &ChessMove, depth: usize) {
        if depth == 0 {
                println!("{}", node.move_text);
        } else {
                for child in &node.children {
                        display_moves_one_step_ahead(child, depth - 1);
                }
        }
}


fn main() {
        // env::set_var("RUST_BACKTRACE", "full");
        println!("Enter a pgn file to get started: ");

        let path = get_file_by_path();
        let formatted_game_matrix = read_file(path).unwrap();

        let mut opening_book = OpeningBook {
                root: ChessMove::new("root", Vec::new()),
        };

        // INIT GAME TREE
        for game in &formatted_game_matrix {
                build_subtree_tree_for_game(&mut opening_book.root, game);
        }
        // display_moves_one_step_ahead(&opening_book.root, 1);


        // let mut current_node = &opening_book.root;
        // let mut next_move: Option<String> = None;
        // for child in current_node.children.iter() {
        //         println!("{}", child.move_text);
        // }
        println!("Opening book: {:#?}", opening_book);

        // // Update frequencies while considering previous moves
        // update_frequencies(&mut opening_book.root, &moves);
        // let mut root = GameNode::init(init_stats(&formatted_game_matrix), formatted_game_matrix.len());
        // let view_perspective = ViewPerspective::White(String::from("white"));
        //
        // GameTreeNavigator::build_tree(&mut root, &formatted_game_matrix, &view_perspective);
        // println!("value of root: {:?}", root);
        //
        // let mut navigator = GameTreeNavigator::new(root);
        //
        // cli::run_cli(&mut navigator, &formatted_game_matrix);

}
