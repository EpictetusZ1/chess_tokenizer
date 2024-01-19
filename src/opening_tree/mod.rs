pub mod build;

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
    pub stats: Option<Stats>,
    pub frequency: u16,
    pub children: HashMap<String, GameNode> // Key is the next move
}

impl GameNode {
    pub fn new(ply: u16, init_stats: Option<Stats>) -> GameNode {
        GameNode {
            ply,
            stats: init_stats,
            frequency: 1,
            children: HashMap::new(),
        }
    }

    pub fn add_or_update_child(&mut self, mov: &str, game_result: GameResult) -> &mut Self {
        self.children.entry(mov.to_string())
            .and_modify(|child| {
                // If move exists increment freq
                child.frequency += 1;
                child.handle_stats(&game_result);
            })
            .or_insert_with(|| {
                let mut new_node = GameNode::new(self.ply + 1, None);
                new_node.handle_stats(&game_result);
                new_node
            });
            // .or_insert_with(|| *GameNode::new(self.ply + 1).handle_stats(&game_result) );

        self.children.get_mut(mov).unwrap()
    }

    pub fn handle_stats(&mut self, game_result: &GameResult) -> &mut Self {
        if let Some(stats) = &mut self.stats {
            match game_result {
                GameResult::W => stats.white -= 1,
                GameResult::B => stats.black -= 1,
                GameResult::D => stats.draws -= 1,
            }
        }
        self
    }


    pub fn get_children(&self) -> Option<&HashMap<String, GameNode>> {
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
