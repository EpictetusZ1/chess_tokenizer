use crate::{Game, GameResult};
use crate::opening_tree::{GameNode, ViewPerspective};
use crate::stats::Stats;

pub fn build_tree(root_node: &mut GameNode, formatted_game_matrix: &Vec<Game>, max_moves: &usize, view_perspective: &ViewPerspective) {

    for game in formatted_game_matrix {
        add_moves_to_node(root_node, &game.moves, 0, *max_moves, &game.result);
    }
}

fn add_moves_to_node(current_node: &mut GameNode, moves: &[String], current_depth: usize, max_depth: usize, game_result: &GameResult) {
    if current_depth >= max_depth || current_depth >= moves.len() {
        return;
    }

    let mov = &moves[current_depth];
    let child_node = current_node.add_or_update_child(mov, *game_result, &current_node.stats.clone());
    add_moves_to_node(child_node, moves, current_depth + 1, max_depth, game_result);
}
