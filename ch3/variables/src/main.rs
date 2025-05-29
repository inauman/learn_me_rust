fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in second is: {THREE_HOURS_IN_SECONDS}");

    let x = 5;

    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("THe value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");

    let mut spaces1 = "   ";
    spaces1 = spaces1.len();
    println!("The number of spaces is: {spaces1}");
}
