mod recoverable;
use std::error::Error;
use recoverable::{last_char_of_first_line, read_first_line, read_username_from_file};

fn main() -> Result<(), Box<dyn Error>>{
    match read_username_from_file() {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("Error reading username: {:?}", e),
    }

    match read_first_line() {
        Ok(contents) => {
            match last_char_of_first_line(&contents) {
                Some(last_char) => println!("last_char: {}", last_char),
                None => println!("No first line or empty line in file"),
            }
        }
        Err(e) => println!("Error reading file: {:?}", e),
    }
    Ok(())
}
