#![allow(dead_code)]
use crate::game_parser::{process_games, split_games};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub mod cli;
mod format_output;
mod game_parser;
pub mod opening_tree;
pub mod stats;

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
pub fn read_file(file_path: String) -> Result<Vec<Game>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    // let contents = fs::read_to_string("games/lichess_EpictetusZ1_2024-01-17.pgn")?;

    let games = split_games(&contents);

    Ok(process_games(games))
    // return only the first 10 games
    // Ok(process_games(games).into_iter().take(2).collect())
}
