pub mod traverse;

use std::collections::HashMap;


#[derive(Debug)]
pub struct GameNode {
    pub ply: u16,
    // pub ply_stats: Stats,
    pub frequency: u16,
    pub children: HashMap<String, GameNode> // Key is the next move
}

impl GameNode {
    pub fn new() -> GameNode {
        // let default = Stats {
        //     wins: 0,
        //     losses: 0,
        //     draws: 0,
        // };

        GameNode {
            ply: 0,
            // ply_stats: default,
            frequency: 1,
            children: HashMap::new(),
        }
    }

    fn check_child(&self, new_ply: &str) -> bool {
        self.children.contains_key(new_ply)
    }

    // Since we are only going to check the parent node, I can use a single hashmap (not nested ones)
    // since the information about the previous ply is solely contained within the parent node, there is no risk of collision
    pub fn add_child(&mut self, new_ply: &str, new_node: GameNode) -> &mut GameNode {
        if self.check_child(&new_ply) {
            let updated = self.children.get_mut(new_ply).unwrap();
            // This seems to be working correctly, there are 234 1. e4 moves in the lichess (my games) file, and the freq of e4 is 234 in the tree
            updated.frequency += 1;
            updated
        } else {
            // Insert the new GameNode into the children hashmap
            self.children.insert(String::from(new_ply), new_node);
            self.children.get_mut(new_ply).unwrap()
        }
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
