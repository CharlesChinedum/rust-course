use std::fs;

fn string_test(a: String, b: &String, c: &str) {
    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn main() {
    string_test(
        "red".to_string(),
        &String::from("green"),
        String::from("blue").as_str(),
    );

    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            println!("File content: {}", text.len());
        }

        Err(error) => {
            println!("Failed to read: {}", error);
        }
    }
}
