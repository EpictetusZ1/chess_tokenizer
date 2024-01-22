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
            Self::add_moves_to_node(root_node, &game.moves, 0, 3, &game.result, game_id);
        }
    }

    pub fn add_moves_to_node(current_node: &mut GameNode, moves: &[String], current_depth: usize, max_depth: usize, game_result: &GameResult, game_id: usize) {
        if current_depth >= max_depth || current_depth >= moves.len() {
            return;
        }

        let mov = &moves[current_depth];
        let child_node = current_node.add_or_update_child(mov, game_id, *game_result, &current_node.stats.clone());
        Self::add_moves_to_node(child_node, moves, current_depth + 1, max_depth, game_result, game_id);
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
            for (game_id, game) in formatted_game_matrix.iter().enumerate() {
                // Self::add_moves_to_node(current_node, &game.moves, path_length,  max_depth, &game.result, game_id);
                Self::add_moves_to_node(current_node, &game.moves, current_node.ply as usize, current_node.ply as usize + max_depth, &game.result, game_id);
            }
        }
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
