use std::fs;

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            println!("File content: {}", text.len());
        }

        Err(error) => {
            println!("Failed to read: {}", error);
        }
    }
}
