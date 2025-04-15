use std::fs;
use std::io::Error;

fn main() {
    // let text = fs::read_to_string("logs.txt");

    // println!("{:#?}", text);

    match divide(5.0, 3.0) {
        Ok(reult) => {
            println!("Result: {}", reult);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Division by zero"))
    } else {
        Ok(a / b)
    }
}
