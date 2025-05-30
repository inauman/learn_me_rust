fn main() {
    let s1 = String::from("hello");
    println!("s1: {s1}");

    let s2 = s1.clone();
    println!("s2: {s2}");
    println!("s1: {s1}");

    let mut s = String::from("Hello");
    s.push_str(", World! Good morning!");
    println!("s: {s}");

    println!("--------------------------------");

    let s1 = String::from("Hi there!");
    takes_onwership(s1);
    //println!("s1: {s1}");

    let x1 = 5;
    makes_copy(x1);
    println!("x1: {x1}");
    
    println!("--------------------------------");
    let s1= gives_onwership();
    println!("s1: {s1}");

    println!("--------------------------------");
    let s2 = String::from("Helloo");
    let s3 = takes_and_gives_back(s2);
    println!("s3: {s3}");

    println!("--------------------------------");
    let s1 = String::from("Hello");
    let (s2, length) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, length);


}


fn takes_onwership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn gives_onwership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}