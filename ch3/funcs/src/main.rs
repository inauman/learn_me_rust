fn another_function(x: i8) {
    println!("Another function. {x}");
}

fn main() {
    println!("Hello, world!");
    another_function(5);
    another_function(100);
    mix_function(5, 'm');
    mix_function(10, 'f');

    //statements and expressions

    let x = {
        let y = 6;
        y + 1
    };

    println!("The value of x is: {x}");
    let y = five();
    println!("The value of y:five is: {y}");

    let z = plus_one(5);
    println!("The value of z:plus_one is: {z}");
}

fn mix_function(value: i32, unit_label: char) {
    println!("The measurement is: {unit_label} - {value}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}