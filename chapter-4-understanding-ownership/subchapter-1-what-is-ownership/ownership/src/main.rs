// A function that takes the ownership as the data is stored in the heap
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

// A function that makes a copy as the data is stored in the stack
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

// Some functions that take the ownership as the data is stored in the heap, but also give it back
fn takes_and_gives_back_ownership(some_string: String) -> String {
    println!("{some_string}");
    some_string
}

fn str_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn main() {
    // Variable scope
    // s is not valid here as it is not yet declared
    {
        let _s = "Hello";
        // s is valid from this point forward
    }
    // s is not valid anymore as it is out of scope

    // The String type as an example of heap stored data (size not known at compile time)
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{s}");

    // Default move (or shallow copy) of heap data (inexpensive)
    let s1 = String::from("Hi!");
    let s2 = s1;
    // Will not work, because the heap part of the string is moved to s2
    // println!("{s1}");
    println!("{s2}");

    // Explicit deep copy of heap data (expensive)
    let s1 = String::from("Hi!");
    let s2 = s1.clone();
    // Both work since the heap part of the string is actually copied
    println!("{s1}");
    println!("{s2}");

    // Default deep copy of stack data (inexpensive)
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Ownership
    let s = String::from("Hi!");
    takes_ownership(s);
    // Will not work, because the ownership is transferred to the function
    // println!("{s}");

    let x = 5;
    makes_copy(x);
    // Works since the data is copied by default
    println!("{x}");

    let mut s = String::from("Hi!");
    s = takes_and_gives_back_ownership(s);
    // Works since the ownership is transferred back, but is very cluttered
    println!("{s}");

    let s = String::from("Hi!");
    let (s, len) = str_len(s);
    // Works since the ownership is transferred back, but is even more cluttered
    println!("{s}");
    println!("{len}");
    // Thankfully references exist
}
