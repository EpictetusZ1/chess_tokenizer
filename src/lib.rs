#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::game_parser::{parse_game_data, parse_result, split_games, process_games};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use crate::opening_tree::GameNode;
use crate::opening_tree::build::build_tree;

pub mod tokenizer;
mod game_parser;
pub mod opening_tree;
mod stats;


#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    W,
    B,
    D,
}

#[derive(Debug)]
pub struct Ply {
    number: u32,
    white_move: String,
    black_move: String,
}

#[derive(Debug)]
pub struct Game {
    pub tags: HashMap<String, String>,
    pub moves: Vec<String>,
    pub result: GameResult,
}

// TODO: Might need to add a sanitize function to clean up: comments, annotations, engine evaluations, etc
pub fn read_file() -> Result<Vec<Game>, Box<dyn Error>> {
    // let contents = fs::read_to_string("games/bad.pgn")?;
    // let contents = fs::read_to_string("games/two.pgn")?;
    let contents = fs::read_to_string("games/lichess_EpictetusZ1_2024-01-17.pgn")?;

    let games = split_games(&contents);

    Ok(process_games(games))
}


#[cfg(test)]
mod tests {
    use crate::game_parser::build_game;
    use crate::opening_tree::ViewPerspective;
    use super::*;

    #[test]
    fn test_game_moves_length() {
        let game = read_and_process_game("games/bad.pgn");

        assert_eq!(
            game.moves.len(),
            81,
            "The length of game moves should be 82"
        );
    }

    fn read_and_process_game(file_path: &str) -> Game {
        // Read the file and process it (simplified for demonstration)
        let contents = fs::read_to_string(file_path).unwrap();
        let games = split_games(&contents);
        let (tags, moves) = parse_game_data(&games[0]);
        let last_move = moves.last().unwrap();
        let parts: Vec<&str> = last_move.split_whitespace().collect();
        let result = parse_result(parts.last().unwrap());

        build_game(tags.to_vec(), moves.to_vec(), result)
    }
    use super::*;

    #[test]
    fn test_correct_first_ply() {
        let formatted_game_matrix = read_file().unwrap();
        let mut root = GameNode::new(0);
        let max_moves = 2;
        let view_perspective = ViewPerspective::White(String::from("white"));

        build_tree(&mut root, &formatted_game_matrix, &max_moves, &view_perspective);

        let expected_moves = ["d3", "d4", "e4", "Nf3", "e3", "f4", "g4", "b3", "c4", "Nc3", "g3"];
        let root_child_keys = root.get_child_keys();

        for mov in &expected_moves {
            assert!(root_child_keys.contains(&mov.to_string()), "Move {} not found in root children", mov);
        }
    }
}