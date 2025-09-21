struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello World!");

    let a = 12;
    let b = "{}";
    println!("a is {} {}", b, a);

    let user1 = User {
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("User1: username: {}, email: {}, sign_in_count: {}, active: {}", user1.username, user1.email, user1.sign_in_count, user1.active);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {}, {}, {}", tup.0, tup.1, tup.2);
}