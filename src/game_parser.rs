use crate::{split_moves, GameResult};

pub fn parse_game_data(contents: &str) -> (Vec<&str>, Vec<String>) {
    let mut tags = vec![];
    let mut moves = vec![];
    let mut reading_moves = false;

    for chunk in contents.lines() {
        if chunk.starts_with('[') {
            //  '[' Lines are tags
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
        "*" => GameResult::P, // Game postponed or something else like arbiter ended
        _ => panic!("Unexpected game result string: {}", result_str),
    }
}
