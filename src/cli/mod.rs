use std::io;


pub fn get_file_by_path() -> String {
    let mut file_path = String::new();

    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");
    file_path.trim().to_string() // Trim the newline character at the end and return the string
}
