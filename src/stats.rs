use crate::GameResult;

#[derive(Debug)]
pub struct Stats {
    pub wins: u16,
    pub losses: u16,
    pub draws: u16,
}

// TODO: Start over and get each piece working correctly before trying to implement more than I can handle

impl Stats {
    pub fn increment_stats(&mut self, new_stats: &Stats) {
        self.wins += new_stats.wins;
        self.losses += new_stats.losses;
        self.draws += new_stats.draws;
    }
}

// Can be shared by Game and Stats
pub trait IsWinner {
    fn update_stats(&self, is_winner: bool, result: &GameResult) -> Stats {
        let mut update_stats = Stats {
            wins: 0,
            losses: 0,
            draws: 0,
        };
        if is_winner {
            match result {
                GameResult::W => update_stats.wins += 1,
                GameResult::B => update_stats.wins += 1,
                GameResult::D => update_stats.draws += 1,
                GameResult::P => (),
            }
        } else {
            match result {
                GameResult::W => update_stats.losses += 1,
                GameResult::B => update_stats.losses += 1,
                GameResult::D => update_stats.draws += 1,
                GameResult::P => (),
            }
        }
        update_stats
    }

    fn is_correct_winner(&self, game_result: &GameResult, player_username: &str, white: &str, black: &str) ->  Stats {
        // let user_won = self.user_is_winner(game_result, player_username);
        let user_won = match game_result {
            GameResult::W => player_username == white,
            GameResult::B => player_username == black,
            GameResult::D | GameResult::P => false,
        };

        self.update_stats(user_won, game_result)
    }
}