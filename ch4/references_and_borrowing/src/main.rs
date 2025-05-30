fn main() {
    println!("Hello, world!");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hola");
    change(&mut s);
    println!("s: {s}");
    {
        let r1 = &mut s;
        println!("r1: {r1}");
    }
    let r2 = &mut s;
    println!("r2: {r2}");

    mut_immut_fn();
    ref_scope_fn();
    value_scope_fn();
    let dangle_ref = dangle();
    println!("dangle_ref: {dangle_ref}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn mut_immut_fn() {
    let s = String::from("hellooo");
    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s;
    println!("r1: {r1}");
    println!("r2: {r2}");
}

fn ref_scope_fn() {
    let mut s = String::from("hello from ref_scope_fn");

    let r1 = &s;
    let r2 = &s;
    println!("r1: {r1}");
    println!("r2: {r2}");
    println!("r1: {r1}");

    let r3 = &mut s;
    println!("r3: {r3}");
    
}

fn value_scope_fn() {
    let s = String::from("hello from value_scope_fn");

    let r1 = s.clone();
    let r2 = s;
    println!("r1: {r1}");
    println!("r2: {r2}");
    
}

fn dangle() -> String {
    let s = String::from("hello from dangle");
    s
}