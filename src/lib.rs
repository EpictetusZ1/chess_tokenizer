use crate::game_parser::{process_games, split_games};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub mod cli;
mod format_output;
mod game_parser;
pub mod opening_tree;

#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    W,
    B,
    D,
}

#[derive(Debug)]
pub struct Game {
    pub tags: HashMap<String, String>,
    pub moves: Vec<String>,
    pub result: GameResult,
}

pub fn read_file(file_path: String) -> Result<Vec<Game>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let games = split_games(&contents);

    Ok(process_games(games))
}
