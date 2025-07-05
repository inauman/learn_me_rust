fn main() {
    println!("\n--------------------------------");
    let median = exercise_1_median();
    match median {
        Some(m) => println!("Median: {:?}", m),
        None => println!("No median"),
    }
    println!("--------------------------------");
}

fn exercise_1_median() -> Option<f64> {
    //let mut v: Vec<i32> = vec![];
    //let mut v: Vec<i32> = vec![3, 2, 1, 5, 4];
    let mut v: Vec<i32> = vec![3, 2, 1, 5, 4, 6];

    v.sort();
    let count = v.len();

    let median = match count {
        0 => None,
        n if n % 2 == 1 => Some(v[n/2] as f64),
        n => {
            let mid1 = v[n/2-1] as f64;
            let mid2 = v[n/2] as f64;
            Some((mid1 + mid2) / 2.0)

        }
    };
    median
}
