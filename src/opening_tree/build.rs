use crate::Game;
use crate::opening_tree::{GameNode, ViewPerspective};

// TODO: One idea would be to set the stats for the first node, then decrement each set of new stats based on what the game result is
pub fn build_tree(root_node: &mut GameNode, formatted_game_matrix: &Vec<Game>, max_moves: &usize, view_perspective: &ViewPerspective) {

    for (game_index, game) in formatted_game_matrix.iter().enumerate() {
        let mut current_node =  &mut *root_node; // I don't think this is the correct way to build the tree

        for ply_number in 0..*max_moves {
            if let Some(mov) = game.moves.get(ply_number) {
                current_node = current_node.add_or_update_child(mov, game.result);
            }
        }
    }
}

