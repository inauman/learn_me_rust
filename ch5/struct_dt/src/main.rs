struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let name = String::from("MN");
    let mut user1 = User {
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
    println!("--------------------------------");
    // update the email
    user1.email = String::from("another@example.com");
    println!("email: {}", user1.email);
    println!("--------------------------------");
    // create a new user with the build_user function
    let user2 = build_user(String::from("user2r@example.com"), String::from("user2_username"));
    println!("user2 username: {}", user2.username);
    println!("user2 email: {}", user2.email);
    println!("user2 active: {}", user2.active);
    println!("user2 sign_in_count: {}", user2.sign_in_count);
}

fn build_user(email: String, username: String) -> User {

    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
