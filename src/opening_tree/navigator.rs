use crate::opening_tree::GameNode;

#[derive(Debug)]
pub struct GameTreeNavigator {
    root: GameNode,
    pub path: Vec<String>, // Stack to keep track of the path
}


impl GameTreeNavigator {
    pub fn new(root: GameNode) -> Self {
        Self {
            root,
            path: Vec::new(),
        }
    }

    pub fn current_node(&mut self) -> &mut GameNode {
        let mut node = &mut self.root;
        for mov_key in &self.path {
            node = node.children.get_mut(mov_key).expect("Invalid path");
        }
        node
    }

    pub fn get_root_mut(&mut self) -> &mut GameNode {
        &mut self.root
    }

    pub fn move_to_node(&mut self, mov_key: &str) {
        self.path.push(mov_key.to_string());
    }

    pub fn go_back(&mut self) {
        self.path.pop();
    }
}
