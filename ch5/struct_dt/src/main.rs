struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let name = String::from("MN");
    let user1 = User {
        active: true,
        username: String::from("nauman09"),
        email: String::from("nauman09@gmail.com"),
        sign_in_count: 1,
    };

    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("active: {}", user1.active);
    println!("sign_in_count: {}", {user1.sign_in_count} );
    println!("name: {name}");
}
