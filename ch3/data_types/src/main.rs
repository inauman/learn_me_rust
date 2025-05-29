fn main() {
    let sum = 5 + 10;
    println!("The sum is: {sum}");

    let truncated = -5 / 3;
    println!("The truncated value is: {truncated}");

    // use floor divsion for -5 and 3
    let floored = ((-5.0 / 3.0) as f64).floor();
    println!("The floored value is: {floored}");

    let t = true;
    let f: bool = false;
    println!("The value of t and f are: {t} and {f}");

    let c = 'z';
    println!("The value of c is: {c}");

    let z: char = '😻';
    println!("The value of z is: {z}");

    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

}
