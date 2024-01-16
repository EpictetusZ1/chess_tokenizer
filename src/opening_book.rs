
struct Opening {
    moves_sequence: Vec<String>,
}

struct OpeningTree {
    opening: Opening,
    children: Vec<OpeningTree>,
}

impl OpeningTree {
    pub fn new(opening: Opening) -> OpeningTree {
        OpeningTree {
            opening,
            children: Vec::new(),
        }
    }
}


// This data structure is a tree of openings
// But should it instead be a tensor of openings?
// probably a tree
