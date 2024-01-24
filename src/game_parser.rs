use crate::{Game, GameResult};
use std::collections::HashMap;

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
pub fn parse_game_data(contents: &str) -> (Vec<&str>, Vec<String>) {
    let mut tags = vec![];
    let mut moves = vec![];
    let mut reading_moves = false;

    for chunk in contents.lines() {
        if chunk.starts_with('[') {
            //  '[]' Lines are tags
            tags.push(chunk);
            reading_moves = false;
        } else if chunk.trim().is_empty() && !reading_moves {
            // An empty line after tags indicates the start of moves
            reading_moves = true;
        } else if reading_moves || !chunk.trim().is_empty() {
            // Non-empty lines AFTER the first empty line are considered moves
            moves.push(chunk);
        }
    }

    let new_moves = split_moves(&moves);

    (tags, new_moves)
}

pub fn clean_tag(raw_tag: &str) -> String {
    raw_tag
        .replace("[", "")
        .replace("]", "")
        .replace("\"", "")
        .replace("/", "")
}

pub fn split_tags(tag: &str) -> (String, String) {
    let parts: Vec<&str> = tag.split_whitespace().collect();
    if parts.len() >= 2 {
        let key = clean_tag(parts[0]);
        let value = parts[1..].join(" "); // Needed in case there is whitespace in tag value

        (key, clean_tag(&value))
    } else {
        panic!("Tag is not in the expected format: {}", tag)
    }
}

pub fn parse_result(result_str: &str) -> GameResult {
    match result_str {
        "1-0" => GameResult::W,
        "0-1" => GameResult::B,
        "0.5-0.5" | "1/2-1/2" => GameResult::D,
        _ => panic!("Unexpected game result string: {}", result_str),
    }
}

pub fn split_moves(moves: &[&str]) -> Vec<String> {
    let mut split_moves = Vec::new();

    for &move_line in moves {
        let words = move_line.split_whitespace();
        for word in words {
            // Split the word if move number and a move are together with no space
            if let Some((_number, move_)) = word.split_once('.') {
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

pub fn process_games(games: Vec<String>) -> Vec<Game> {
    let mut all_games: Vec<Game> = Vec::new();

    for game in games {
        let (tags, moves) = parse_game_data(&game);
        if let Some(game) = handle_game_result(&tags, &moves) {
            all_games.push(game);
        }
    }

    all_games
}

pub fn handle_game_result(tags: &[&str], moves: &[String]) -> Option<Game> {
    match moves.last() {
        Some(last_move) => {
            match last_move.split_whitespace().last() {
                Some(result) => {
                    let game = build_game(tags.to_vec(), moves.to_vec(), parse_result(result));
                    Some(game) // Return the Game object
                }
                None => {
                    eprintln!("Could not find the result in the last move.");
                    None // Return None as no Game can be constructed
                }
            }
        }
        None => {
            eprintln!("There are no moves to analyze.");
            None // Return None as no Game can be constructed
        }
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
