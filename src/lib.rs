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
            println!("The result is: {result}");
        } else {
            println!("Could not find the result in the last move."); // maybe handle this case with U?
        }
    } else {
        println!("There are no moves to analyze.");
    }

    Ok(())
}

pub enum GameResult {
    W,
    B,
    D,
}

pub struct Game {
    tags: HashMap<String, String>,
    moves: HashMap<usize, String>,
    result: GameResult
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

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
}