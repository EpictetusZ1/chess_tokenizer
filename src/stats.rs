use crate::{Game, GameResult};

#[derive(Debug, Clone, Copy)] // Add this if Stats only contains copyable types
pub struct Stats {
    pub white: u16,
    pub black: u16,
    pub draws: u16,
}

pub fn init_stats(formatted_game_matrix: &Vec<Game>) -> Stats {
    // Loop through all the games, and just increase each stat according to what happened
    let mut all_stats = Stats {
        white: 0,
        black: 0,
        draws: 0,
    };

    for game in formatted_game_matrix {
        match game.result {
            GameResult::W => all_stats.white += 1,
            GameResult::B => all_stats.black += 1,
            GameResult::D => all_stats.draws += 1,
        }
    }

    all_stats
}
