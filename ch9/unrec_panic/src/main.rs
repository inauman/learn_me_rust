mod recoverable;

use recoverable::test_recoverable_v2_closure;

fn main() {
    //panic!("crash and burn");
    //let v = vec![1, 2, 3];
    //v[99];
    test_recoverable_v2_closure();
}
