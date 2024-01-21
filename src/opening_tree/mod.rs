pub mod build;
pub mod navigator;

use std::collections::HashMap;
use crate::{GameResult, Ply};
use crate::stats::Stats;


pub enum ViewPerspective {
    White(String),
    Black(String)
}

#[derive(Debug)]
pub struct GameNode {
    pub ply: u16,
    pub stats: Stats,
    pub frequency: u16,
    pub children: HashMap<String, GameNode> // Key is the next move
}

impl GameNode {
    pub fn init(init_stats: Stats, total_freq: usize) -> GameNode {
        GameNode {
            ply: 0,
            stats: init_stats,
            frequency: total_freq as u16,
            children: HashMap::new(),
        }
    }

    pub fn new(ply: u16, prev_stats: &Stats) -> GameNode {
        GameNode {
            ply,
            stats: *prev_stats,
            frequency: 1,
            children: HashMap::new(),
        }
    }

    pub fn add_or_update_child(&mut self, mov: &str, game_result: GameResult, prev_node_stats: &Stats) -> &mut Self {
        self.children.entry(mov.to_string())
            .and_modify(|child| {
                // If move exists increment freq
                child.frequency += 1;
                child.handle_stats(&game_result);
            })
            .or_insert_with(|| {
                let mut new_node = GameNode::new(self.ply + 1, prev_node_stats);
                new_node.handle_stats(&game_result);
                new_node
            });

        self.children.get_mut(mov).unwrap()
    }

    pub fn handle_stats(&mut self, game_result: &GameResult) -> &mut Self {
        match game_result {
            GameResult::W => self.stats.white -= 1,
            GameResult::B => self.stats.black -= 1,
            GameResult::D => self.stats.draws -= 1,
        }
        self
    }

    pub fn get_child(&self) -> Option<&HashMap<String, GameNode>> {
        Some(&self.children)
    }

    pub fn get_child_keys(&self) -> Vec<String>  {
        let has_child = Some(&self.children);
        let mut child_keys = vec![];

        if let Some(child) = has_child {
            child_keys = self.children.keys().cloned().collect()
        }

        child_keys
    }

    // TODO: Write add game result method
}
