use chess_tokenizer::read_file;
// use chess_tokenizer::opening_book;
use chess_tokenizer::opening_book::{GameNode};


fn main() {
    let formatted_game_matrix = read_file().unwrap();
    let mut root  = GameNode::new();
    let max_moves = 1;

    for move_number in 0..max_moves {

        for (game_index, game) in formatted_game_matrix.iter().enumerate() {
            let mut current_node = &mut root;
            let testing = current_node.get_first_child();

            // Each item here is a single chess game
            let curr_game_ref = formatted_game_matrix.get(game_index);

            // println!("curr game ref?: {:?}", curr_game_ref);
            if let Some(q) = curr_game_ref {
                let curr_move_ref = q.moves.get(move_number);

                if let Some(p) = curr_move_ref {
                    let new_node = GameNode {
                        ply: move_number as u16,
                        // ply_stats: node_stats,
                        frequency: 1,
                        children: Default::default(),
                    };

                    current_node = current_node.add_child(p.clone(), new_node);
                    println!("Current node value is {:?}", current_node);
                    println!("Move at game number: {:?} is {:?}", move_number, p);
                }
            }
        }
    }
}

