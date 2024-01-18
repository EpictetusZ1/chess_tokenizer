use crate::Game;
use crate::opening_tree::GameNode;


// TODO: Add logic to check if the node already exists, and if it does, increment it and move it down


pub fn traverse_tree(root_node: &mut GameNode, formatted_game_matrix: Vec<Game>, max_moves: usize) {
    for move_number in 0..max_moves {

        for (game_index, game) in formatted_game_matrix.iter().enumerate() {
            let mut current_node = &mut *root_node;
            let testing = current_node.get_child_keys();

            let curr_game_ref = formatted_game_matrix.get(game_index);

            if let Some(q) = curr_game_ref {
                let curr_move_ref = q.moves.get(move_number);

                if let Some(p) = curr_move_ref {
                    let new_node = GameNode {
                        ply: move_number as u16,
                        // ply_stats: node_stats,
                        frequency: 1,
                        children: Default::default(),
                    };

                    current_node = current_node.add_child(p, new_node);
                    // println!("Current node value is {:?}", current_node);
                    // println!("Move at game number: {:?} is {:?}", move_number, p);
                }
            }
        }
    }
}


// ["d3", "d4", "e4", "Nf3", "e3", "f4", "g4", "b3", "c4", "Nc3", "g3"]