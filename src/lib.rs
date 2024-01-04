use std::collections::HashMap;
use std::error::Error;
use std::fs;
// use std::env;

// Game is made up of:
// tags
// whitespace line (line break char), but might not always have this?
// move list
// score at end of move list

pub fn read_file() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("Alexei Shirov_vs_Garry Kasparov_1997.__.__.pgn")?;

    let (tags, moves) = split_game_data(&contents);

    if let Some(last_move) = moves.last() {
        let parts: Vec<&str> = last_move.split_whitespace().collect();
        if let Some(result) = parts.last() {
            let game = build_game(tags, &moves, parse_result(result));
            println!("DATA BUILT: {:?}", game);
        } else {
            println!("Could not find the result in the last move."); // maybe handle this case with U?
        }
    } else {
        println!("There are no moves to analyze.");
    }

    Ok(())
}

pub fn split_game_data(contents: &str) -> (Vec<&str>, Vec<&str>) {
    let mut tags= vec![];
    let mut moves = vec![];

    let mut hit_empty_line = false;

    let data: Vec<&str> = contents
        .lines()
        .collect();

    for chunk in data {
        if chunk.is_empty() {
            hit_empty_line = true;
        } else {
            if hit_empty_line {
                moves.push(chunk);
            } else {
                tags.push(chunk);
            }
        }
    }

    (tags, moves)
}

#[derive(Debug)]
pub enum GameResult {
    W,
    B,
    D,
}

#[derive(Debug)]
pub struct Game {
    tags: HashMap<String, String>,
    moves: HashMap<usize, String>,
    result: GameResult
}

pub fn split_tags(tag: &str) -> (String, String) {
    let parts: Vec<&str> = tag.split_whitespace().collect();
    if parts.len() >= 2 {
        (parts[0].to_string(), parts[1].to_string())
    } else {
        panic!("Tag is not in the expected format: {}", tag)
    }}

pub fn build_game(tags: Vec<&str>, move_list: &Vec<&str>, g_result: GameResult) -> Game  {
    let mut format_tags: HashMap<String, String> = HashMap::new();
    let mut format_moves: HashMap<usize, String> = HashMap::new();

    for tag in tags {
        let (key, value) = split_tags(tag);
        format_tags.insert(key, value);
    }

    for (index, m) in move_list.iter().enumerate() {
        format_moves.insert(index + 1, m.to_string());
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
        _ => panic!("Unexpected game result string: {}", result_str),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
}