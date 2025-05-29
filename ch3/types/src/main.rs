fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let guess: u32 = "42".parse().expect("Not a number");
    println!("The value of guess is: {guess}");

    let x: i32 = 5;
    println!("The value of x is: {x}");

    let y: i32 = 0x2A;
    println!("The value of y is: {y}");

    let z: u32 = b'A'.into();
    println!("The value of z is: {z}");

    let mut a1: u8 = 253;
    a1 = a1 + 1;
    println!("The value of a1 is: {a1}");

    //floating point numbers
    let _xf = 2.0; //f64
    let _yf: f32 = 3.0; //f32

    let truncated = -5/3;
    println!("The value of truncated is: {truncated}");
}
