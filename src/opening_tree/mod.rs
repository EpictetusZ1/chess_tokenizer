pub mod build;

use std::collections::HashMap;


#[derive(Debug)]
pub struct GameNode {
    pub ply: u16,
    pub frequency: u16,
    pub children: HashMap<String, GameNode> // Key is the next move
}

impl GameNode {
    pub fn new() -> GameNode {
        GameNode {
            ply: 0,
            frequency: 1,
            children: HashMap::new(),
        }
    }

    pub fn add_or_update_child(&mut self, mov: &str) -> &mut Self {
        self.children.entry(mov.to_string())
            .and_modify(|child| {
                // If move exists increment freq
                child.frequency += 1;
            })
            .or_insert_with(|| GameNode {
                ply: self.ply + 1,
                frequency: 1,
                children: HashMap::new(),
            });

        self.children.get_mut(mov).unwrap()
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
