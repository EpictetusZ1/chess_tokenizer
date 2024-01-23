use crate::stats::Stats;
use crate::{GameResult, Ply};
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug)]
pub enum ViewPerspective {
    White(String),
    Black(String),
}

#[derive(Debug)]
pub struct FormattedOutput {
    pub mov: String,
    pub freq: u16,
}

use crate::Game;

#[derive(Debug, Clone)]
pub struct ChessMove {
    pub move_text: String,
    pub children: Vec<ChessMove>,
    pub frequency: usize,
    pub prev_moves: Vec<String>,
}

#[derive(Debug)]
pub struct OpeningBook<'a> {
    pub root: ChessMove,
    pub current_node: Option<ChessMove>,
    pub node_stack: Vec<ChessMove>,
    pub games_data: &'a [Game], // Reference to the games data
}

impl<'a> OpeningBook<'a> {
    pub fn new(root: ChessMove, games_data: &'a [Game]) -> OpeningBook<'a> {
        OpeningBook {
            root,
            current_node: None,
            node_stack: Vec::new(),
            games_data,
        }
    }

    // method to set the current node and push the previous node to the stack
    pub fn set_node(&mut self, node: ChessMove) {
        if let Some(current_node) = self.current_node.take() {
            self.node_stack.push(current_node);
        }
        self.current_node = Some(node);
    }

    pub fn reset_view(&mut self) {
        // Reset the view level to the root
        self.current_node = None;
    }

    pub fn navigate_up(&mut self) -> Result<(), &'static str> {
        if let Some(prev_node) = self.node_stack.pop() {
            self.current_node = Some(prev_node);
            Ok(())
        } else {
            Err("Already at the root node")
        }
    }
    pub fn navigate_down(&mut self, move_text: &str) -> Result<(), &'static str> {
        if let Some(current) = &self.current_node {
            self.node_stack.push(current.clone());
        }

        if let Some(mut child) = self.current_node.as_ref().and_then(|node| {
            node.children
                .iter()
                .find(|child| child.move_text == move_text)
                .cloned()
        }) {
            // Check if the child needs more expansion and expand if necessary
            if child.children.is_empty() {
                child.expand_subtree(self.games_data); // Pass the reference to the games data
            }

            self.current_node = Some(child);
            Ok(())
        } else {
            self.node_stack.pop();
            Err("No child found with the given move.")
        }
    }
    // pub fn navigate_down(&mut self, move_text: &str) -> Result<(), &'static str> {
    //     if let Some(current) = &self.current_node {
    //         // Push a clone of the current node onto the stack before changing it
    //         self.node_stack.push(current.clone());
    //     }
    //
    //     if let Some(child) = self.current_node.as_ref().and_then(|node| {
    //         node.children.iter().find(|child| child.move_text == move_text).cloned()
    //     }) {
    //         self.current_node = Some(child);
    //         Ok(())
    //     } else {
    //         // If no child is found, remove the last pushed node as we didn't actually navigate down
    //         self.node_stack.pop();
    //         Err("No child found with the given move.")
    //     }
    // }
}

impl ChessMove {
    pub fn new(move_text: &str, prev_moves: Vec<String>) -> ChessMove {
        ChessMove {
            move_text: move_text.to_string(),
            children: Vec::new(),
            frequency: 0,
            prev_moves,
        }
    }

    pub fn build_subtree_from_depth(&mut self, moves: &[String], start_depth: usize) {
        let mut current_node = self;

        for (i, move_text) in moves.iter().enumerate().skip(start_depth) {
            if i >= start_depth + 2 {
                break; // Limit to two moves ahead from the starting depth
            }

            let child_index = current_node
                .children
                .iter()
                .position(|c| c.move_text == *move_text);

            if let Some(index) = child_index {
                // If the child exists, increment its frequency
                current_node.children[index].frequency += 1;
            } else {
                // If the child does not exist, create a new one
                let mut new_child = ChessMove::new(move_text, {
                    let mut prev_moves = current_node.prev_moves.clone();
                    prev_moves.push(move_text.to_string());
                    prev_moves
                });
                new_child.frequency = 1; // Initialize frequency for new child
                current_node.children.push(new_child);
            }

            // Update current_node to point to the child node
            current_node = current_node
                .children
                .iter_mut()
                .find(|c| c.move_text == *move_text)
                .unwrap();
        }
    }

    pub fn expand_subtree(&mut self, games: &[Game]) {
        let current_depth = self.prev_moves.len();

        for game in games {
            let moves = &game.moves; // Assuming moves is a Vec<String>

            let mut current_node = &mut *self; // Create a mutable reference to self

            for move_text in moves.iter().skip(current_depth) {
                let child_index = current_node
                    .children
                    .iter()
                    .position(|c| c.move_text == *move_text);

                if let Some(index) = child_index {
                    current_node.children[index].frequency += 1;
                    current_node = &mut current_node.children[index];
                } else {
                    let mut new_child = ChessMove::new(move_text, {
                        let mut prev_moves = current_node.prev_moves.clone();
                        prev_moves.push(move_text.to_string());
                        prev_moves
                    });
                    new_child.frequency = 1; // Initialize frequency for new child
                    current_node.children.push(new_child);
                    current_node = current_node.children.last_mut().unwrap();
                }
            }
        }
    }
    // pub fn expand_subtree(&mut self, games: &[Game]) {
    //     let current_depth = self.prev_moves.len();
    //
    //     for game in games {
    //         // Assuming `game.moves` is a Vec<String> representing the moves in the game
    //         let moves: Vec<String> = game.moves.clone();
    //
    //         // Now pass the moves slice to build_subtree_from_depth
    //         self.build_subtree_from_depth(&moves, current_depth);
    //     }
    // }

    pub fn build_subtree_tree_for_game(&mut self, game: &Game, start_depth: usize) {
        // Extract moves from the game and build the tree
        let moves: Vec<String> = game.moves.clone(); // Assuming moves field in Game is a Vec<String>

        let mut current_node = self; // Clone root before entering the loop

        for (i, move_text) in moves.iter().enumerate() {
            // Find the child node with the matching move text or create a new one if not found
            let child_index = current_node
                .children
                .iter()
                .position(|c| c.move_text == *move_text);
            let child = match child_index {
                Some(index) => &mut current_node.children[index],
                None => {
                    let new_child = ChessMove::new(move_text, {
                        let mut prev_moves = current_node.prev_moves.clone();
                        prev_moves.push(move_text.to_string());
                        prev_moves
                    });
                    current_node.children.push(new_child);
                    current_node.children.last_mut().unwrap()
                }
            };

            // Update the frequency of the current move
            child.frequency += 1;

            // Set the current node to the child for the next iteration
            current_node = child;

            // Limit to one move ahead
            if i >= 1 {
                break;
            }
        }
    }

    pub fn get_child_keys(&self) -> Vec<FormattedOutput> {
        let mut child_keys = self
            .children
            .iter()
            .map(|child| FormattedOutput {
                mov: child.move_text.clone(),
                freq: child.frequency as u16,
            })
            .collect::<Vec<_>>();

        // Sort by frequency in descending order
        child_keys.sort_by(|a, b| b.freq.cmp(&a.freq));

        child_keys
    }

    pub fn current_node(&self, path: &[String]) -> Option<&ChessMove> {
        let mut node = self;
        for mov_key in path {
            if let Some(child) = node
                .children
                .iter()
                .find(|child| &child.move_text == mov_key)
            {
                node = child;
            } else {
                return None;
            }
        }
        Some(node)
    }
}
