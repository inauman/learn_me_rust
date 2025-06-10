#[derive(Debug)]

enum UsState {
    Alaska,
    Alabama,

}

impl UsState {
    fn existed_in(&self, year: i16) -> bool {
        match self {
            UsState::Alabama => year >= 1861,
            _ => false,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn penny_count(coin: Coin) -> u8 {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    count
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn dice_roll_fn() {
    let dice_roll = 7;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other),
        _ => (),
    }
}

fn add_fancy_hat() {
    println!("Adding fancy hat");
}

fn remove_fancy_hat() {
    println!("Removing fancy hat");
}

fn move_player(num_spaces: u8) {
    println!("Moving {} spaces", num_spaces);
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1999) {
            Some(format!("{state:?} is pretty old for America!"))
        }
        else {
            Some(format!("{state:?} is relatively new!"))
        }
    }
    else {
        None
    }
}

fn describe_state_quarter_v2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    }
    else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old for America!"))
    }
    else {
        Some(format!("{state:?} is relatively new!"))
    }
}

fn describe_state_quarter_v3(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old for America!"))
    }
    else {
        Some(format!("{state:?} is relatively new!"))
    }
}
fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("It's worth {} cents!", value_in_cents(coin));

    let coin = Coin::Dime;
    let penny_count = penny_count(coin);
    println!("penny_count: {}", penny_count);

    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);

    let none = plus_one(None);
    println!("none: {:?}", none);

    dice_roll_fn();
}
