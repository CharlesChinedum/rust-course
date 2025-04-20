use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            let error_logs = extract_errors(text.as_str());

            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => println!("Wrote errors.txt"),
                Err(error) => {
                    println!("Writing of errors.txt failed: {}", error)
                }
            }
        }

        Err(error) => {
            println!("Failed to read: {}", error);
        }
    }
}
