#![allow(unused_imports)]
use std::env;
use opening_tree::cli::{get_file_by_path};
use opening_tree::{cli, Game, read_file};
use opening_tree::opening_tree::{GameNode, ViewPerspective};
use opening_tree::opening_tree::navigator::GameTreeNavigator;
use opening_tree::opening_tree::test::{ ChessMove, OpeningBook};
use opening_tree::stats::init_stats;


fn main() {
        // env::set_var("RUST_BACKTRACE", "full");
        println!("Enter a pgn file to get started: ");

        let path = get_file_by_path();
        let formatted_game_matrix = read_file(path).unwrap();

        let mut opening_book = OpeningBook::new(ChessMove::new("root", Vec::new()),);

        // INIT GAME TREE
        for game in &formatted_game_matrix {
                opening_book.root.build_subtree_tree_for_game(game);
        }

        for child in opening_book.root.children.iter() {
                println!("move: {}, freq: {}", child.move_text, child.frequency);
        }

        opening_book.set_node(opening_book.root.clone());
        println!("root: {:?}", opening_book.root);
        cli::run_cli(&mut opening_book);



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
 }
