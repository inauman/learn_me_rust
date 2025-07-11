use std::fs::{self, File};
use std::io::{self, Error, ErrorKind, Read};

/// Demonstrates basic error handling with match and nested matches.
pub fn test_recoverable_v1_match() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };
}

/// Demonstrates error handling using closures with unwrap_or_else.
pub fn test_recoverable_v2_closure() {
    let greeting_file_result = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

/// Demonstrates using expect for error handling with a custom message.
pub fn test_recoverable_v3_unwrap() {
    let greeting_file =
        File::open("hello.txt").expect("\nhello.txt should be included in this project.\n\n");
}

#[allow(dead_code)]
/// Version 1: Verbose error handling with match.
pub fn read_username_from_file_v1() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

#[allow(dead_code)]
/// Version 2: Using the ? operator for simpler propagation.
pub fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

#[allow(dead_code)]
/// Version 4: Chained ? operators for conciseness.
pub fn read_username_from_file_v4() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

/// Final version: Uses fs::read_to_string for simplicity.
pub fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

/// Reads the contents of hello.txt (note: reads entire file, not just first line).
pub fn read_first_line() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut first_line = String::new();
    file.read_to_string(&mut first_line)?;
    Ok(first_line)
}

/// Extracts the last character of the first line from the given text.
pub fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
