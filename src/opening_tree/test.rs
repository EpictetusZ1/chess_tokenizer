use crate::Game;
use crate::opening_tree::GameNode;

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
}

pub fn build_subtree_tree_for_game(root: &mut ChessMove, game: &Game) {
    // Extract moves from the game and build the tree
    let moves: Vec<String> = game.moves.clone(); // Assuming moves field in Game is a Vec<String>

    let mut current_node = root; // Clone root before entering the loop

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


