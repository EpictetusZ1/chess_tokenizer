use std::collections::HashMap;

struct Stats {
    wins: u16,
    losses: u16,
    draws: u16,
}

struct GameNode {
    ply: String,
    ply_stats: Stats,
    frequency: u16,
    children: HashMap<str, GameNode> // Key is ply count
}

impl GameNode {
    pub fn new(ply: String) -> GameNode {
        let default = Stats {
            wins: 0,
            losses: 0,
            draws: 0,
        };

        GameNode {
            ply,
            ply_stats: default,
            frequency: 1,
            children: HashMap::new(),
        }
    }

    // Since we are only going to check the parent node, I can use a single hashmap (not nested ones)
    // since the information about the previous ply is solely contained within the parent node, there is no risk of collision
    // pub fn add_child(&self, new_ply: String) -> GameNode {
    //     // Create the new GameNode, then add it to the parents children hashmap
    //
    // }

    // TODO: Write add game result method
}

// A Tree is a collection of nodes that are connected by edges
// the topmost node is called the root node, the nodes below it are called child nodes

// Requirements:
// Take a Vec of Game Structs,

