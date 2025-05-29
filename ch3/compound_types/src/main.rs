//import io
use std::io;
fn main() {
    let tup: (i32, f64, u8, char) = (500, 6.4, 1, 'a');
    let (x, y, z, a) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of a is: {a}");

    //access tuple by index
    let x = tup.0;
    let y = tup.1;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    // array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The first element is {first}");

    let second = a[1];
    println!("The second element is {second}");

    let third = a[2];
    println!("The third element is {third}");

    println!("Please enter an array index:");

    let mut index = String::new();
    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    let index: usize = index.trim().parse()
    .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
    
}