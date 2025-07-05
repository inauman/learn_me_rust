mod exercise1;
use crate::exercise1::{exercise_1_median, exercise_1_mode};
fn main() {
    println!("\n--------------------------------");
    let median = exercise_1_median();
    match median {
        Some(m) => println!("Median: {:?}", m),
        None => println!("No median"),
    }
    println!("--------------------------------");

    let mode = exercise_1_mode();
    match mode {
        Some(m) => println!("Mode: {:?}", m),
        None => println!("No mode"),
    }
}
