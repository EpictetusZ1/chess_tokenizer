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
    let contents = fs::read_to_string("games/Alexei Shirov_vs_Garry Kasparov_1997.__.__.pgn")?;

    let (tags, moves) = split_game_data(&contents);

    if let Some(last_move) = moves.last() {
        let parts: Vec<&str> = last_move.split_whitespace().collect();
        if let Some(result) = parts.last() {
            let game = build_game(tags, moves, parse_result(result));
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
    P,
}

#[derive(Debug)]
pub struct Game {
    tags: HashMap<String, String>,
    moves: Vec<String>,
    result: GameResult,
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
    use super::*;

    // #[test]
}
