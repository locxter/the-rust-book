// A simple enum as a set of known possible values
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Rust also allows us to store data in each enum variant (with different data types for each variant)
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// And implementations are also possible
enum _Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl _Message {
    fn _call(&self) {
        // Some code
    }
}

fn main() {
    // Enums are used just like any other data type
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}", four);
    println!("{:?}", six);
    // With stored data it looks like this
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);
    // The Option enum is Rust's safe variant of a null value (either Some or None)
    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;
    let _absent_char: Option<char> = None;
    // This will not work as it is unsafe (some_number needs to converted to i32 first and errors need to be handeled)
    // let sum = 5 + some_number;
    // This is where the match construct comes into play
}
