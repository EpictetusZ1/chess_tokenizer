use crate::{Game, GameResult};
use crate::opening_tree::{GameNode, ViewPerspective};
use crate::stats::Stats;

// TODO: One idea would be to set the stats for the first node, then decrement each set of new stats based on what the game result is
pub fn build_tree(root_node: &mut GameNode, formatted_game_matrix: &Vec<Game>, max_moves: &usize, view_perspective: &ViewPerspective) {

    for (game_index, game) in formatted_game_matrix.iter().enumerate() {
        let mut current_node =  &mut *root_node; // I don't think this is the correct way to build the tree

        for ply_number in 0..*max_moves {
            if let Some(mov) = game.moves.get(ply_number) {
                current_node = current_node.add_or_update_child(mov, game.result, &current_node.stats.clone());
            }
        }
    }
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
            GameResult::W =>   all_stats.white += 1,
            GameResult::B =>   all_stats.black += 1,
            GameResult::D =>   all_stats.draws += 1,
        }
    }

    all_stats
}


// pub fn init_freq(game_vec_length: u16) -> u16 {
//     game_vec_length
// }