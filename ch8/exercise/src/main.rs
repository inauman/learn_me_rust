use std::collections::HashMap;
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

fn exercise_1_median() -> Option<f64> {
    //let mut v: Vec<i32> = vec![];
    //let mut v: Vec<i32> = vec![3, 2, 1, 5, 4];
    let mut v: Vec<i32> = vec![3, 2, 1, 5, 4, 6];

    v.sort();
    let count = v.len();

    let median = match count {
        0 => None,
        n if n % 2 == 1 => Some(v[n / 2] as f64),
        n => {
            let mid1 = v[n / 2 - 1] as f64;
            let mid2 = v[n / 2] as f64;
            Some((mid1 + mid2) / 2.0)
        }
    };
    median
}

fn exercise_1_mode() -> Option<i32> {
    let mut v: Vec<i32> = vec![1, 2, 2, 3, 3, 3, 4, 4, 1, 1, 5, 6, 1, 7, 1, 1];
    let mut counts = HashMap::new();

    for &value in &v {
        let count = counts.entry(value).or_insert(0);
        *count += 1;
    }

    let mut mode = None;
    let mut max_count = 0;
    for (number, count) in &counts {
        if *count > max_count {
            max_count = *count;
            mode = Some(*number);
        }
    }
    mode
}
