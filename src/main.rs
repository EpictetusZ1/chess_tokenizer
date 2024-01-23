use opening_tree::cli::get_file_by_path;
use opening_tree::opening_tree::{ChessMove, OpeningBook};
use opening_tree::{cli, read_file, Game};

fn main() {
    println!("Enter a pgn file to get started: ");

    let path = get_file_by_path();
    let formatted_game_matrix = read_file(path).unwrap();

    let mut opening_book =
        OpeningBook::new(ChessMove::new("root", Vec::new()), &formatted_game_matrix);

    // // INIT GAME TREE
    for game in &formatted_game_matrix {
        opening_book.root.build_subtree_tree_for_game(game, 0);
    }
    opening_book.set_node(opening_book.root.clone());
    cli::run_cli(&mut opening_book);
}
