#![allow(dead_code)]
#![allow(unused_variables)]
use crate::game_parser::{parse_game_data, parse_result, split_tags, split_games};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

mod game_parser;
pub mod tokenizer;
mod opening_book;

#[derive(Debug)]
pub enum GameResult {
    W,
    B,
    D,
    P,
}

#[derive(Debug)]
pub struct Ply {
    number: u32,
    white_move: String,
    black_move: String,
}

#[derive(Debug)]
pub struct Game {
    tags: HashMap<String, String>,
    moves: Vec<String>,
    result: GameResult,
}

// TODO: Might need to add a sanitize function to clean up: comments, annotations, engine evaluations, etc

pub fn read_file() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("games/Alexei Shirov_vs_Garry Kasparov_1997.__.__.pgn")?;
    // let contents = fs::read_to_string("games/Multi_1.pgn")?;
    // let contents = fs::read_to_string("games/bad.pgn")?;

    let games = split_games(&contents);

    process_games(games);
    Ok(())
}

pub fn process_games(games: Vec<String>) {
    for game in games {
        let (tags, moves) = parse_game_data(&game);
        handle_game_result(&tags, &moves);
    }
}

pub fn handle_game_result(tags: &[&str], moves: &[String]) {
    match moves.last() {
        Some(last_move) => {
            match last_move.split_whitespace().last() {
                Some(result) => {
                    let game = build_game(tags.to_vec(), moves.to_vec(), parse_result(result));

                    let mut game_tokens = tokenizer::Tokenizer::new(game.moves.join(" "));
                    game_tokens.tokenize();
                    println!("Game Tokens: {:?}", game_tokens);

                    // println!("Built Game Data: {:?}", game);
                },
                None => eprintln!("Could not find the result in the last move.")
            }
        },
        None => eprintln!("There are no moves to analyze.")
    }
}

pub fn build_game(tags: Vec<&str>, move_list: Vec<String>, g_result: GameResult) -> Game {
    let mut format_tags: HashMap<String, String> = HashMap::new();
    let mut format_moves: Vec<String> = Vec::new();

    for &tag in tags.iter() {
        let (key, value) = split_tags(tag);
        format_tags.insert(key, value);
    }

    // Iterate over all except last, that is the game result
    for m in move_list[..move_list.len() - 1].iter() {
        format_moves.push(m.to_string());
    }

    Game {
        tags: format_tags,
        moves: format_moves,
        result: g_result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_moves_length() {
        // Assuming 'read_and_process_game' is a function that reads the PGN file,
        // parses the data, and returns a Game struct
        let game = read_and_process_game("games/Alexei Shirov_vs_Garry Kasparov_1997.__.__.pgn");

        // Assert that the length of the moves vector is 76
        assert_eq!(
            game.moves.len(),
            76,
            "The length of game moves should be 76"
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
}
