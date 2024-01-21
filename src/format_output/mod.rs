use crate::opening_tree::FormattedOutput;

pub fn print_possible_moves(moves: &[FormattedOutput]) {
    let mut lines = Vec::new();

    // Collect formatted strings into the vector
    for formatted_output in moves {
        let line = format!("Move: {:<7} Freq: {}", formatted_output.mov, formatted_output.freq);
        lines.push(line);
    }

    // Calculate the number of columns (each with 10 rows max)
    let cols = (lines.len() as f32 / 10.0).ceil() as usize;

    // Print the lines in a structured multi-column format
    for row in 0..10 {
        for col in 0..cols {
            let index = col * 10 + row;
            if index < lines.len() {
                print!("{:<25}   ", lines[index]);
            } else {
                // Print spaces to align columns if there's no more data in this column
                print!("{:<25}", "");
            }
        }
        println!(); // New line at the end of each row
    }
}
