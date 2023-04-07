// A function using an immutable reference
fn str_len(s: &String) -> usize {
    s.len()
}

// A function using a mutable reference
fn modify_str(s: &mut String) {
    s.push_str(", world!");
}

// Will not work since s goes out of scope at the end of the function and therefore can not be referenced later on
//fn gen_str() -> &String {
//    let s = String::from("Hi!");
//    &s
//}

// Works since the ownership of s is transferred and it therefore does not go out of scope at the end of the function
fn gen_str() -> String {
    let s = String::from("Hi!");
    s
}

fn main() {
    // The same function using a reference for borrowing the data instead of owning it (multiple simultaneous references
    // per variable possible)
    let s = String::from("Hi!");
    let len = str_len(&s);
    println!("{s}");
    println!("{len}");
    // Much better and easier to read/understand

    // Mutable references for changing data without owning it (only one reference per variable possible at a time and
    // exclusive with immutable ones)
    let mut s = String::from("Hello");
    modify_str(&mut s);
    println!("{s}");
    // In conclusion, you can have either one mutable reference or any number of immutables references to a variable at
    // any given time

    // Not worries about invalid dangling references
    let s = gen_str();
    println!("{s}");
}
