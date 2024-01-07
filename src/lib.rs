use std::collections::HashMap;
use std::error::Error;
use std::fs;
// use std::env;

pub mod tokenizer;

#[derive(Debug)]
pub enum GameResult {
    W,
    B,
    D,
    P,
}

#[derive(Debug)]
pub struct Game {
    tags: HashMap<String, String>,
    moves: Vec<String>,
    result: GameResult,
}

pub fn read_file() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("games/Multi_1.pgn")?;
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

pub fn handle_game_result(tags: &[&str], moves: &[&str]) {
    if let Some(last_move) = moves.last() {
        let parts: Vec<&str> = last_move.split_whitespace().collect();
        if let Some(result) = parts.last() {

            let game = build_game(tags.to_vec(), moves.to_vec(), parse_result(result));

        } else {
            eprintln!("Could not find the result in the last move.");
        }
    } else {
        eprintln!("There are no moves to analyze.");
    }
}

pub fn split_games(file_contents: &str) -> Vec<String> {
    let mut games: Vec<String> = Vec::new();
    let mut cur_game: Vec<&str> = Vec::new();

    let mut consecutive_empty_lines = 0;

    for line in file_contents.lines() {
        if line.trim().is_empty() {
            consecutive_empty_lines += 1;

            // Two empty lines indicate end of current game
            if consecutive_empty_lines == 2 && !cur_game.is_empty() {
                games.push(cur_game.join("\n")); // Join the current game's lines into a single string
                cur_game.clear(); // Prepare to start a new game
            }
        } else {
            consecutive_empty_lines = 0; // Reset count when a non-empty line is encountered
            cur_game.push(line); // Add the line to the current game
        }
    }

    // Handle the last game in the file (if it doesn't end with two empty lines)
    if !cur_game.is_empty() {
        games.push(cur_game.join("\n")); // Join the last game's lines into a single string
    }

    games
}

pub fn parse_game_data(contents: &str) -> (Vec<&str>, Vec<&str>) {
    let mut tags = vec![];
    let mut moves = vec![];
    let mut reading_moves = false;

    for chunk in contents.lines() {
        if chunk.starts_with('[') { // Lines starting with '[' are considered tags
            tags.push(chunk);
            reading_moves = false;
        } else if chunk.trim().is_empty() && !reading_moves {
            // An empty line after tags indicates the start of moves
            reading_moves = true;
        } else if reading_moves || !chunk.trim().is_empty() {
            // Non-empty lines after the first empty line are considered moves
            moves.push(chunk);
        }
    }

    (tags, moves)
}

pub fn clean_tag(raw_tag: &str) -> String {
    raw_tag.replace("[", "")
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

pub fn build_game(tags: Vec<&str>, move_list: Vec<&str>, g_result: GameResult) -> Game  {
    let mut format_tags: HashMap<String, String> = HashMap::new();
    let mut format_moves: Vec<String> = Vec::new();

    for &tag in tags.iter() {
        let (key, value) = split_tags(tag);
        format_tags.insert(key, value);
    }

    // Iterate over all except last, that is the game result
    for &m in move_list[..move_list.len() - 1].iter() {
        format_moves.push(m.to_string());
    }

    Game {
        tags: format_tags,
        moves: format_moves,
        result: g_result,
    }
}

pub fn parse_result(result_str: &str) -> GameResult {
    match result_str {
        "1-0" => GameResult::W,
        "0-1" => GameResult::B,
        "0.5-0.5" | "1/2-1/2" => GameResult::D,
        "*" => GameResult::P, // Game postponed or something else like arbiter ended
        _ => panic!("Unexpected game result string: {}", result_str),
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
}
