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
    println!("--------------------------------");

   // create a new user with update feature
   let user3 = User {
    email: String::from("user3@example.com"),
    ..user2
   };

   println!("user3 username: {}", user3.username);
   println!("user3 email: {}", user3.email);
   println!("user3 active: {}", user3.active);
   println!("user3 sign_in_count: {}", user3.sign_in_count);
   println!("--------------------------------");

   // try to print the user2 username
   println!("user2 active: {}", user2.email);

   tuple_structs();

}

fn build_user(email: String, username: String) -> User {

    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(10,20,30);

    println!("black: {} - {} - {}", black.0, black.1, black.2);
    println!("origin: {} - {} - {}", origin.0, origin.1, origin.2);
    println!("--------------------------------");

   // destructuring
   let Point(x,y,z) = origin;
   println!("x: {} - y: {} - z: {}", x,y,z);
   println!("--------------------------------");

   // destructuring with pattern matching
   let Color(r,g,b) = black;
   println!("r: {} - g: {} - b: {}", r,g,b);
   println!("--------------------------------");

   // destructuring with pattern matching
}