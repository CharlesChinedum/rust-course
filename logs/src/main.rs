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

    let email = String::from("test@test.com");

    match validate_email(email) {
        Ok(..) => println!("Valid email"),

        Err(error) => {
            println!("Error: {}", error)
        }
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("Invalid email address"))
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Division by zero"))
    } else {
        Ok(a / b)
    }
}
