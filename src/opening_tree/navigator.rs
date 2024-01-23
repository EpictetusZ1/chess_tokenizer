use crate::{Game, GameResult};
use crate::opening_tree::{GameNode, ViewPerspective};

#[derive(Debug)]
pub struct GameTreeNavigator {
    root: GameNode,
    pub path: Vec<String>, // Stack to keep track of the path
}


impl GameTreeNavigator {
    pub fn new(root: GameNode) -> Self {
        Self {
            root,
            path: Vec::new(),
        }
    }

    pub fn build_tree(root_node: &mut GameNode, formatted_game_matrix: &Vec<Game>, view_perspective: &ViewPerspective) {
        for (game_id, game) in formatted_game_matrix.iter().enumerate() {
            Self::add_moves_to_node(root_node, &game.moves, 0, 5, &game.result, game_id);
        }
    }

    pub fn add_moves_to_node(current_node: &mut GameNode, moves: &[String], current_depth: usize, max_depth: usize, game_result: &GameResult, game_id: usize) -> Vec<String> {
        if current_depth >= max_depth || current_depth >= moves.len() {
            return current_node.get_traversal_path()
        }

        let traversal_path = current_node.get_traversal_path();
        let mov = &moves[current_depth];
        let child_node = current_node.add_or_update_child(mov, game_id, *game_result, &current_node.stats.clone(), traversal_path, moves);
        Self::add_moves_to_node(child_node, moves, current_depth + 2, max_depth, game_result, game_id);
        return current_node.get_traversal_path()
    }

    pub fn update_tree_if_needed(navigator: &mut GameTreeNavigator, formatted_game_matrix: &[Game], view_perspective: &ViewPerspective, max_depth: usize) {
        let path = navigator.current_path().clone();
        let path_length = navigator.path.len();
        let mut current_node = navigator.get_root_mut().clone();

        for mov_key in path {
            if let Some(node) = current_node.children.get_mut(&mov_key) {
                current_node = node.clone();
            } else {
                // If the path leads to a non-existent node, return
                return;
            }
        }

        // Check if the current node's children are fully built
        if current_node.children_are_not_built() {
            let current_ply = current_node.ply;
            for (game_id, game) in formatted_game_matrix.iter().enumerate() {
                if game.moves.len() > current_ply as usize {
                    // Use the current depth of the node and max_depth for expansion
                    let depth_to_expand = std::cmp::max(path_length, current_ply as usize) + max_depth;
                    let updated_path = Self::add_moves_to_node(&mut current_node, &game.moves, current_ply as usize, depth_to_expand, &game.result, game_id);
                    navigator.update_current_path(updated_path);
                }
            }
        }
    }



    pub fn set_root_mut(&mut self, new_root: &mut GameNode) {
        self.root = new_root.clone();
    }

    pub fn current_node(&mut self) -> &mut GameNode {
        let mut node = &mut self.root;
        for mov_key in &self.path {
            node = node.children.get_mut(mov_key).expect("Invalid path");
        }
        node
    }

    pub fn current_path(&self) -> &Vec<String> {
        &self.path
    }

    pub fn update_current_path(&mut self, new_path: Vec<String>) {
        self.path = new_path;
    }

    pub fn get_root_mut(&mut self) -> &mut GameNode {
        &mut self.root
    }

    pub fn move_to_node(&mut self, mov_key: &str) {
        self.path.push(mov_key.to_string());
    }

    pub fn go_back(&mut self) {
        self.path.pop();
    }
}
