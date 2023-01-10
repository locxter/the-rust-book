// A simple struct grouping together related data with named fields (unlike tuples where the fields are not named)
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Some tuple structs without named fields but as distinctly named types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// A unit-like struct without any fields (similar to an empty tuple () aka unit)
struct AlwaysEqual;

// A function to create an instance of User
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // or username: username,
        email,    // or email: email
        sign_in_count: 1,
    }
}

fn main() {
    // The order of the fields is irrelevant as they are named
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };
    // Access is done using the dot notation (just like tuples but with named fields)
    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
    user1.email = String::from("someuser@example.com");
    println!("{}", user1.email);
    // Use our build user function to make things easier
    let user2 = build_user(
        String::from("someoneelse@example.com"),
        String::from("someoneelse"),
    );
    println!("{}", user2.active);
    println!("{}", user2.username);
    println!("{}", user2.email);
    println!("{}", user2.sign_in_count);
    // Struct update syntax to include another instances values (borrows the heap data and therefore renders user1 unusable)
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}", user3.active);
    println!("{}", user3.username);
    println!("{}", user3.email);
    println!("{}", user3.sign_in_count);
    // For tuple structs field access is the same as for normal tuples
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", black.1);
    println!("{}", black.2);
    println!("{}", origin.0);
    println!("{}", origin.1);
    println!("{}", origin.2);
    // Not useful without any implemented traits (aka methods)
    let _always_equal = AlwaysEqual;
}
