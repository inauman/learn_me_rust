fn main() {
    intro();
    str2();
}

fn intro() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let mut hello = String::from("Hello Sam");

    hello.push_str(" World");
    hello.push_str("!!!");

    println!("{}", hello);

    let s1 = String::from("@Hello");
    let s2 = String::from(" World@");

    let s3 = s1 + &s2;

    println!("{}", s3);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let t4 = format!("{t1}-{t2}-{t3}");
    println!("{t4}");
    let t5 = &t4[0..1];
    println!("{}", t5);

    let hello1 = "Здравствуйте";
    let answer = &hello1[0..2];
    println!("{}", answer);
}

fn str2() {
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
