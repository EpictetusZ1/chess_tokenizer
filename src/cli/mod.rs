use std::f32::consts::E;
use std::{io, process};
use crate::cli::UserInput::{Exit, NextNode, PreviousNode};
use crate::format_output::print_possible_moves;
use crate::Game;
use crate::opening_tree::{GameNode, ViewPerspective};
use crate::opening_tree::navigator::GameTreeNavigator;
use crate::opening_tree::test::OpeningBook;


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

pub fn run_cli(opening_book: &mut OpeningBook) {
    loop {
        if opening_book.current_node.is_none() {
            // Handle the case where no node is selected yet
            let mut user_input = String::new();
            println!("No node selected. Enter a move or 'exit' to quit:");
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read input");

            match UserInput::match_args(user_input.trim().to_string()) {
                PreviousNode => {
                    if let Err(err) = opening_book.navigate_up() {
                        println!("{}", err); // Print the error message
                    }
                    // Ok::<(), &str>(()) // Specify the type parameter here
                   ()
                },
                Exit => {
                    println!("Exiting");
                    process::exit(1);
                },
                NextNode(move_str) => {
                    let next_move = opening_book.navigate_down(&move_str);

                    println!("next_move: {:?}", next_move);
                    // Ok::<(), &str>(()) // Specify the type parameter here
                    ()
                }
            }
            continue;
        }
        let mut current_node = opening_book.current_node.as_ref().unwrap();
        println!("Current node: {:?}", current_node);
        let possible_moves = current_node.get_child_keys();
        print_possible_moves(&possible_moves);

        // Get user input
        let mut user_input = String::new();
        println!("Enter your move, '..' to go back, or 'exit' to quit:");
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read input");

        // Process user input
        match UserInput::match_args(user_input.trim().to_string()) {
            PreviousNode => {
                opening_book.navigate_up().expect("TODO: panic message");
               ()
            },
            Exit => {
                println!("Exiting");
                process::exit(1);
            },
            NextNode(move_str) => {
                let next_move = opening_book.navigate_down(&move_str);
                println!("next_move: {:?}", next_move);
               ()


                // if let Some(child)=  {
                //     opening_book.navigate_down(&move_str).expect("TODO: panic message");
                //     Ok::<(), &str>(()) // Specify the type parameter here
                // } else {
                //     println!("Invalid move: {}", move_str);
                //     Ok::<(), &str>(()) // Specify the type parameter here
                // }
                // if let Some(child) = current_node.children.iter_mut().find(|child| child.move_text == move_str) {
                //     let new_current_node = child;
                //     current_node = new_current_node;
                //     Ok::<(), &str>(()) // Specify the type parameter here
                // } else {
                //     println!("Invalid move: {}", move_str);
                //     Ok::<(), &str>(()) // Specify the type parameter here
                //
                // }
            }
            // NextNode(move_str) => {
            //     if let Some(child) = current_node.children.iter().find(|child| child.move_text == move_str) {
            //         opening_book.navigate_down(&move_str).expect("TODO: panic message");
            //         Ok::<(), &str>(()) // Specify the type parameter here
            //
            //     } else {
            //         println!("Invalid move: {}", move_str);
            //         Ok::<(), &str>(()) // Specify the type parameter here
            //
            //     }
            // },
        }
    }
}
