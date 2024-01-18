pub mod traverse;

use std::collections::HashMap;
// use crate::GameResult;


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

    // Since we are only going to check the parent node, I can use a single hashmap (not nested ones)
    // since the information about the previous ply is solely contained within the parent node, there is no risk of collision
    pub fn add_child(&mut self, new_ply: String, new_node: GameNode) -> &mut GameNode {
        // Insert the new GameNode into the children hashmap
        self.children.insert(new_ply.clone(), new_node);

        // Create the new GameNode, then add it to the parents children hashmap
        self.children.get_mut(&new_ply).unwrap()
    }

    pub fn get_children(&self) -> Option<&HashMap<String, GameNode>> {
        Some(&self.children)
    }

    // -> Option<Vec<String>>
    pub fn get_child_keys(&self) -> Vec<String>  {
        let has_child = Some(&self.children);
        let mut child_keys = vec![];
        // .filter_map(|(key, &val)| if val == value { Some(key) } else { None })
        // let keys: Vec<&str> = map.keys().cloned().collect();

        if let Some(child) = has_child {
            // let child_keys = self.children.iter().filter_map(|(key, value)| Some(key));
             child_keys = self.children.keys().cloned().collect()
        }

        child_keys
    }

    // TODO: Write add game result method
}
