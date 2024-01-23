use std::f32::consts::E;
use std::{io, process};
use crate::cli::UserInput::{Exit, NextNode, PreviousNode};
use crate::format_output::print_possible_moves;
use crate::Game;
use crate::opening_tree::{GameNode, ViewPerspective};
use crate::opening_tree::navigator::GameTreeNavigator;


pub fn get_file_by_path() -> String {
    // let mut file_path = String::new();
    //
    // io::stdin()
    //     .read_line(&mut file_path)
    //     .expect("Failed to read line");
    String::from("games/lichess_EpictetusZ1_2024-01-17.pgn")
    // file_path.trim().to_string() // Trim the newline character at the end and return the string
}


pub enum UserInput {
    NextNode(String),
    PreviousNode,
    Exit,
}

impl UserInput {
    pub fn match_args(input: String) -> UserInput {
        let trimmed_input = input.trim();
        match trimmed_input {
            ".." => PreviousNode,
            _ if trimmed_input.eq_ignore_ascii_case("exit") => Exit,
            _ => NextNode(trimmed_input.to_string()),
        }
    }
}
pub fn run_cli(navigator: &mut GameTreeNavigator, formatted_game_matrix: &[Game]) {
    loop {
        let current_path = navigator.current_path().clone();
        let current_node = navigator.current_node();
        let possible_moves = current_node.get_child_keys();
        println!("Current path: {:?}", current_path);
        print_possible_moves(&possible_moves);

        // Get user input
        let mut user_input = String::new();
        println!("Enter your move, '..' to go back, or 'exit' to quit:");
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read input");

        // Process user input
        match UserInput::match_args(user_input) {
            PreviousNode => navigator.go_back(),
            Exit => {
                println!("Exiting");
                process::exit(1);
            },
            NextNode(move_str) => {
                if current_node.children.contains_key(&move_str) {
                    navigator.move_to_node(&move_str);
                    let new_depth = current_path.len() + 2; // For example, explore 2 moves deeper

                    // GameTreeNavigator::update_tree_if_needed(navigator, formatted_game_matrix, view_perspective, new_depth)
                } else {
                    println!("Invalid move: {}", move_str);
                }
            },
        }
    }
}
