mod recoverable;

//use recoverable::test_recoverable_v3_unwrap;
use recoverable::read_username_from_file;

fn main() {
    //panic!("crash and burn");
    //let v = vec![1, 2, 3];
    //v[99];
    //test_recoverable_v2_closure();
    //test_recoverable_v3_unwrap();
    let username = recoverable::read_username_from_file().unwrap();
    println!("username: {}", username);
}
