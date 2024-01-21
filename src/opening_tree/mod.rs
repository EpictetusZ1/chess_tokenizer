pub mod build;
pub mod navigator;

use std::collections::HashMap;
use crate::{GameResult, Ply};
use crate::stats::Stats;


#[derive(Debug)]
pub enum ViewPerspective {
    White(String),
    Black(String)
}

#[derive(Debug)]
pub struct GameNode {
    pub ply: u16,
    pub stats: Stats,
    pub frequency: u16,
    pub children: HashMap<String, GameNode>, // Key is the next move
    pub children_fully_built: bool,
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
        }
    }

    pub fn new(ply: u16, prev_stats: &Stats) -> GameNode {
        GameNode {
            ply,
            stats: *prev_stats,
            frequency: 1,
            children: HashMap::new(),
            children_fully_built: false,
        }
    }

    pub fn add_or_update_child(&mut self, mov: &str, game_result: GameResult, prev_node_stats: &Stats) -> &mut Self {
        self.children.entry(mov.to_string())
            .and_modify(|child| {
                // If move exists increment freq
                child.frequency += 1;
                // child.handle_stats(&game_result);
            })
            .or_insert_with(|| {
                let mut new_node = GameNode::new(self.ply + 1, prev_node_stats);
                // new_node.handle_stats(&game_result);
                new_node
            });

        self.children.get_mut(mov).unwrap()
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


    pub fn handle_stats(&mut self, game_result: &GameResult) -> &mut Self {
        println!("Current stats are: {:?}", self);
        match game_result {
            GameResult::W => self.stats.white -= 1,
            GameResult::B => self.stats.black -= 1,
            GameResult::D => self.stats.draws -= 1,
        }
        self
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
}
