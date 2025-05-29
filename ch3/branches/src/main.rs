use std::io;
fn main() {
    println!("Please enter a number:");

    let mut number = String::new();       // y + 1.r

    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line");
    let number: i32 = number.trim().parse()
    .expect("Please type a number!");

    if number % 5 == 0&& number % 3 == 0 {
        println!("number is divisible by 5 and 3");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 5 == 0  {
        println!("number is divisible by 5");
    }
    else {
        println!("number is not divisible by 3 or 5");
    }
}
