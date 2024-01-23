use crate::Game;
use crate::opening_tree::{FormattedOutput, GameNode};

#[derive(Debug)]
pub struct Navigate {
    root: GameNode,
    pub path: Vec<String>, // Stack to keep track of the path
}

#[derive(Debug, Clone)]
pub struct ChessMove {
   pub move_text: String,
    pub children: Vec<ChessMove>,
    pub frequency: usize,
    pub prev_moves: Vec<String>,
}

#[derive(Debug)]
pub struct OpeningBook {
    pub root: ChessMove,
    pub current_node: Option<ChessMove>,
    pub node_stack: Vec<ChessMove>,
}

impl OpeningBook {
    pub fn new(root: ChessMove) -> Self {
        OpeningBook {
            root,
            current_node: None,
            node_stack: Vec::new(),
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
        if let Some(node) = self.node_stack.pop() {
            self.current_node = Some(node);
            Ok(())
        } else {
            Err("No parent found.")
        }
    }

    pub fn navigate_down(&mut self, move_text: &str) -> Result<(), &'static str> {
        // Clone the current node and add it to the stack
        if let Some(ref current) = self.current_node {
            self.node_stack.push(current.clone());
        }

        // Find the child node and update the current node
        if let Some(child) = self.current_node.take().and_then(|node| {
            node.children.iter().find(|child| child.move_text == move_text).cloned()
        }) {
            self.current_node = Some(child);
            Ok(())
        } else {
            // If no child is found, remove the current node from the stack since we didn't navigate down
            self.node_stack.pop();
            Err("No child found with the given move.")
        }
    }


    pub fn get_node_by_path(&self, path: &[String]) -> Option<&ChessMove> {
        let mut current_node = &self.root;

        for move_str in path {
            let child = current_node.children.iter().find(|child| child.move_text == *move_str)?;

            current_node = child;
        }

        Some(current_node)
    }
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

    pub fn build_subtree_tree_for_game(&mut self, game: &Game) {
        // Extract moves from the game and build the tree
        let moves: Vec<String> = game.moves.clone(); // Assuming moves field in Game is a Vec<String>

        let mut current_node = self; // Clone root before entering the loop

        for (i, move_text) in moves.iter().enumerate() {
            // Find the child node with the matching move text or create a new one if not found
            let child_index = current_node.children.iter().position(|c| c.move_text == *move_text);
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
        let mut child_keys = self.children.iter().map(|child| {
            FormattedOutput {
                mov: child.move_text.clone(),
                freq: child.frequency as u16,
            }
        }).collect::<Vec<_>>();

        // Sort by frequency in descending order
        child_keys.sort_by(|a, b| b.freq.cmp(&a.freq));

        child_keys
    }

    pub fn current_node(&self, path: &[String]) -> Option<&ChessMove> {
        let mut node = self;
        for mov_key in path {
            if let Some(child) = node.children.iter().find(|child| &child.move_text == mov_key) {
                node = child;
            } else {
                return None;
            }
        }
        Some(node)
    }
}


