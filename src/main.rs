use chess_tokenizer::read_file;
use chess_tokenizer::opening_book;
use chess_tokenizer::opening_book::{GameNode};


fn main() {
    let formatted_game_matrix = read_file().unwrap();
    let username = "EpictetusZ1";

    // println!("Formatted Game Matrix: {:?}", formatted_game_matrix);
    let mut root  = opening_book::GameNode::new();
    // let mut current_stats = Stats { wins: 0, losses: 0, draws: 0 };

    for (game_index, game) in formatted_game_matrix.iter().enumerate() {
        let mut current_node = &mut root;

        for (move_index, mov) in game.moves.iter().enumerate() {
            // let node_stats = Stats {
            //     wins: current_stats.wins,
            //     losses: current_stats.losses,
            //     draws: current_stats.draws,
            // };

            let new_node = GameNode {
                ply: move_index as u16,
                // ply_stats: node_stats,
                frequency: 1,
                children: Default::default(),
            };
            current_node = current_node.add_child(mov.clone(), new_node);
        }
        println!("Current node: {:?}", current_node);
    }
}
