use crate::{Game, GameResult};
use crate::opening_tree::{GameNode, ViewPerspective};
use crate::opening_tree::navigator::GameTreeNavigator;
use crate::stats::Stats;

pub fn build_tree(root_node: &mut GameNode, formatted_game_matrix: &Vec<Game>, max_moves: &usize, view_perspective: &ViewPerspective) {

    for game in formatted_game_matrix {
        add_moves_to_node(root_node, &game.moves, 0, *max_moves, &game.result);
    }
}

// fn add_moves_to_node(current_node: &mut GameNode, moves: &[String], current_depth: usize, game_result: &GameResult) {
//     if current_depth >= moves.len() {
//         return;
//     }
//
//     // Add only the next move in the sequence
//     let mov = &moves[current_depth];
//     current_node.add_or_update_child(mov, *game_result, &current_node.stats.clone());
//
// }
fn add_moves_to_node(current_node: &mut GameNode, moves: &[String], current_depth: usize, max_depth: usize, game_result: &GameResult) {
    if current_depth >= max_depth || current_depth >= moves.len() {
        return;
    }

    let mov = &moves[current_depth];
    let child_node = current_node.add_or_update_child(mov, *game_result, &current_node.stats.clone());
    add_moves_to_node(child_node, moves, current_depth + 1, max_depth, game_result);
}

pub fn update_tree_if_needed(navigator: &mut GameTreeNavigator, formatted_game_matrix: &[Game], view_perspective: &ViewPerspective, max_depth: usize) {
    let path = navigator.current_path().clone();
    let path_length = navigator.path.len();

    let mut current_node = navigator.get_root_mut();
    for mov_key in path {
        if let Some(node) = current_node.children.get_mut(&mov_key) {
            current_node = node;
        } else {
            // If the path leads to a non-existent node, return
            return;
        }
    }

    // Check if the current node's children are fully built
    if current_node.children_are_not_built() {
        for game in formatted_game_matrix {
            add_moves_to_node(current_node, &game.moves, path_length,  max_depth, &game.result);
        }
    }
}

