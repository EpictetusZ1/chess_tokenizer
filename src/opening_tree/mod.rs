pub mod navigator;

use std::collections::{BTreeSet, HashMap, HashSet};
use crate::{GameResult, Ply};
use crate::stats::Stats;


#[derive(Debug)]
pub enum ViewPerspective {
    White(String),
    Black(String)
}

#[derive(Debug, Clone)]
pub struct GameNode {
    pub ply: u16,
    pub stats: Stats,
    pub frequency: u16,
    pub children: HashMap<String, GameNode>, // Key is the next move
    pub children_fully_built: bool,
    pub game_ids: BTreeSet<usize>, // Tracking unique game IDs
    pub move_counters: HashMap<String, u16>, // To store move counts at this position
}

#[derive(Debug)]
pub struct FormattedOutput {
    pub mov: String,
    pub freq: u16,
}

impl GameNode {
    pub fn init(init_stats: Stats, total_freq: usize) -> GameNode {
        GameNode {
            ply: 0,
            stats: init_stats,
            frequency: total_freq as u16,
            children: HashMap::new(),
            children_fully_built: false,
            game_ids: BTreeSet::new(),
            move_counters: HashMap::new(),
        }
    }

    pub fn new(ply: u16, prev_stats: &Stats) -> GameNode {
        GameNode {
            ply,
            stats: *prev_stats,
            frequency: 1,
            children: HashMap::new(),
            children_fully_built: false,
            game_ids: BTreeSet::new(),
            move_counters: HashMap::new(),
        }
    }

    pub fn get_traversal_path(&self) -> Vec<String> {
        let mut traversal_path = Vec::new();
        let mut current_node = self;
        while current_node.ply >= 0 {
            let move_counters = current_node.move_counters.clone();
            let mut move_keys = move_counters.keys().map(|s| s.clone()).collect::<Vec<_>>();
            move_keys.sort_by(|a, b| {
                let count_a = move_counters[a];
                let count_b = move_counters[b];
                count_b.cmp(&count_a) // Reverse the order to sort in descending order
            });
            if !move_keys.is_empty() {
                traversal_path.push(move_keys[0].to_string());
                current_node = current_node.children.get(&move_keys[0]).unwrap();
            } else {
                break; // Exit the loop if move_keys is empty
            }
        }
        traversal_path.reverse();
        traversal_path
    }

    pub fn add_or_update_child(&mut self, mov: &str, game_id: usize, game_result: GameResult, prev_node_stats: &Stats, traversal_path: Vec<String>, moves: &[String]) -> &mut Self {
        println!("Adding or updating child for move '{}'", mov); // Debug output

        println!("moves: {:?}", moves); // Debug output
        println!("traversal_path: {:?}", traversal_path); // Debug output
        let child = self.children.entry(mov.to_string())
            .or_insert_with(|| {
                let mut new_node = GameNode::new(self.ply + 1, prev_node_stats);
                new_node.game_ids.insert(game_id);
                println!("Creating new node for move '{}', initial frequency: 1, game_id: {}", mov, game_id); // Debug output
                new_node
            });

        // Check if the move occurred in the same game before incrementing the frequency
        if child.game_ids.insert(game_id) {
            let mut current_node = &mut *child; // Get a mutable reference to the child node
            let mut matching_moves = 0;

            for (i, mov) in moves.iter().enumerate() {
                if current_node.move_counters.contains_key(mov) {
                    current_node.move_counters.entry(mov.to_string()).and_modify(|freq| {
                        *freq += 1;
                    });
                    matching_moves += 1;
                }

                if i < traversal_path.len() && matching_moves == i + 1 {
                    current_node = current_node.children.get_mut(&traversal_path[i]).unwrap();
                }
            }

            // Increment the frequency only when the move list and traversal path match
            if matching_moves == traversal_path.len() {
                current_node.frequency += 1;
            }
        }
        // child.handle_stats(&game_result);

        child
    }

    pub fn children_are_not_built(&self) -> bool {
        !self.children_fully_built
    }

    pub fn set_children_as_built(&mut self) {
        self.children_fully_built = true;
    }

    pub fn children_are_fully_built(&self, total_games: usize) -> bool {
        for node in self.children.values() {
            if (node.frequency as usize) < total_games {
                return false;
            }
        }
        true
    }


    pub fn get_child_keys(&self) -> Vec<FormattedOutput> {
        let mut child_keys = self.children.iter().map(|(mov, node)| {
            FormattedOutput {
                mov: mov.to_string(),
                freq: node.frequency
            }
        }).collect::<Vec<_>>();

        // Sort by frequency in descending order
        child_keys.sort_by(|a, b| b.freq.cmp(&a.freq));

        child_keys
    }

// pub fn handle_stats(&mut self, game_result: &GameResult) -> &mut Self {
//     println!("Current stats are: {:?}", self);
//     match game_result {
//         GameResult::W => self.stats.white -= 1,
//         GameResult::B => self.stats.black -= 1,
//         GameResult::D => self.stats.draws -= 1,
//     }
//     self
// }
}
