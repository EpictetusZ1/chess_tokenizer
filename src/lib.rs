#![allow(dead_code)]
#![allow(unused_variables)]
use crate::game_parser::{parse_game_data, parse_result, split_tags};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

mod game_parser;
pub mod tokenizer;

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

pub fn handle_game_result(tags: &[&str], moves: &Vec<String>) {
    if let Some(last_move) = moves.last() {
        let parts: Vec<&str> = last_move.split_whitespace().collect();
        if let Some(result) = parts.last() {
            let game = build_game(tags.to_vec(), moves.to_vec(), parse_result(result));
            println!("Built Game Data: {:?}", game);
        } else {
            eprintln!("Could not find the result in the last move.");
        }
    } else {
        eprintln!("There are no moves to analyze.");
    }
}

enum GameState {
    InsideGame,
    OutsideGame,
}

pub fn split_games(file_contents: &str) -> Vec<String> {
    let mut games: Vec<String> = Vec::new();
    let mut cur_game = String::new();
    let mut consecutive_empty_lines = 0;

    for line in file_contents.lines() {
        if line.trim().is_empty() {
            consecutive_empty_lines += 1;
            if consecutive_empty_lines == 2 {
                if !cur_game.is_empty() {
                    games.push(cur_game.trim_end_matches('\n').to_string());
                    cur_game.clear();
                }
                consecutive_empty_lines = 0;
            }
        } else {
            cur_game.push_str(line);
            cur_game.push('\n');
            consecutive_empty_lines = 0;
        }
    }

    if !cur_game.is_empty() {
        games.push(cur_game.trim_end_matches('\n').to_string());
    }

    games
}

pub fn split_moves(moves: &[&str]) -> Vec<String> {
    let mut split_moves = Vec::new();

    for &move_line in moves {
        let words = move_line.split_whitespace();
        for word in words {
            // Split the word if move number and a move are together with no space
            if let Some((number, move_)) = word.split_once('.') {
                // Check if the part after the period isn't empty and push it as a move
                if !move_.is_empty() {
                    split_moves.push(move_.to_string());
                }
            } else {
                // If there's no period, it's a move
                split_moves.push(word.to_string());
            }
        }
    }

    split_moves
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
