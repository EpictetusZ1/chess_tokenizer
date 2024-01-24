use crate::cli::UserInput::{Exit, NextNode, PreviousNode};
use crate::format_output::print_possible_moves;
use crate::opening_tree::OpeningBook;
use std::{io, process};

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
            ".." => PreviousNode,
            _ if trimmed_input.eq_ignore_ascii_case("exit") => Exit,
            _ => NextNode(trimmed_input.to_string()),
        }
    }
}

enum ActionResult {
    Exit,
    Continue,
    Error(String), // Use this variant to handle errors
}

fn process_user_input(input: String, opening_book: &mut OpeningBook) -> ActionResult {
    match UserInput::match_args(input) {
        PreviousNode => match opening_book.navigate_up() {
            Ok(_) => ActionResult::Continue,
            Err(err) => ActionResult::Error(err.to_string()),
        },
        Exit => ActionResult::Exit,
        NextNode(move_str) => match opening_book.navigate_down(&move_str) {
            Ok(_) => ActionResult::Continue,
            Err(err) => ActionResult::Error(err.to_string()),
        },
    }
}

pub fn run_cli(opening_book: &mut OpeningBook) {
    loop {
        let mut user_input = String::new();

        if opening_book.current_node.is_none() {
            println!("No node selected. Enter a move or 'exit' to quit:");
        } else {
            let current_node = opening_book.current_node.as_ref().unwrap();
            println!("Current node: {:?}", current_node.prev_moves);
            let possible_moves = current_node.get_child_keys();
            print_possible_moves(&possible_moves);
            println!("Enter your move, '..' to go back, or 'exit' to quit:");
        }

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");
        match process_user_input(user_input.trim().to_string(), opening_book) {
            ActionResult::Exit => {
                println!("Exiting");
                process::exit(0);
            }
            ActionResult::Continue => (),
            ActionResult::Error(err_msg) => println!("Error: {}", err_msg),
        }
    }
}
