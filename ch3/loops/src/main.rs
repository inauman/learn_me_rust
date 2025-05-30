fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    another_loop_fn();
    while_loop_fn();
    while_loop2_fn();
    for_loop_fn();
}

fn another_loop_fn() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 3 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop_fn() {
    let mut number = 3;

    while number !=0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn while_loop2_fn() {
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn for_loop_fn() {
    let a = [10, 20, 30, 40, 50];

    for i in a {
        println!("the value is: {i}");
    }

    println!("LIFTOFF!!!");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}