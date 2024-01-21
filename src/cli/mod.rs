use std::f32::consts::E;
use std::{io, process};
use crate::cli::UserInput::{Exit, NextNode, PreviousNode};
use crate::opening_tree::GameNode;
use crate::opening_tree::navigator::GameTreeNavigator;


pub fn get_file_by_path() -> String {
    let mut file_path = String::new();

    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");

    file_path.trim().to_string() // Trim the newline character at the end and return the string
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
            ".." => UserInput::PreviousNode,
            _ if trimmed_input.eq_ignore_ascii_case("exit") => UserInput::Exit,
            _ => UserInput::NextNode(trimmed_input.to_string()),
        }
    }
}
pub fn run_cli(navigator: &mut GameTreeNavigator) {
    loop {
        // TODO: Order these by frequency
        // Display the current node and possible moves
        let current_node = navigator.current_node();
        println!("Possible moves: {:?}", current_node.get_child_keys());

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
                    navigator.move_to_node(&move_str)
                } else {
                    println!("Invalid move: {}", move_str);
                }
            },
        }
    }
}
