// Works, but is error prone as the returned index is not connected to the given string in any way
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// A saver variant of the same function using a string slice
fn first_word_slice_output(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// An even better version of this function using string slices for everything
fn first_word_slices_everywhere(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("The quick brown fox");
    // Not an ideal solution since manual state synchronisation is needed
    let w = first_word(&s);
    println!("{s}");
    println!("{w}");

    // String slices (&str) are (often partial) references to strings (as well as the data type of string literals)
    let the = &s[..3]; // or &s[0..3]
    let quick = &s[4..9];
    let brown = &s[10..15];
    let fox = &s[16..]; // or &s[16..s.len()]
    let the_quick_brown_fox = &s[..]; // or &s[0..s.len()]
    let literal = "Hi!";
    println!("{} {} {} {}", the, quick, brown, fox);
    println!("{the_quick_brown_fox}");
    println!("{literal}");

    // Slices are a solution for this problem
    let w = first_word_slice_output(&s);
    println!("{s}");
    // This would generate an compiler error which is exactly what we want
    // s.clear();
    println!("{}", w.len());
    println!("{w}");

    // And also allow us to pass string literals as well as normal strings to functions
    let literal = "Lorem ipsum sit";
    let w = first_word_slices_everywhere(literal);
    println!("{}", w.len());
    println!("{w}");
    let w = first_word_slices_everywhere(&literal[6..]);
    println!("{}", w.len());
    println!("{w}");
    let w = first_word_slices_everywhere(&s[..]);
    println!("{}", w.len());
    println!("{w}");
    // Even this shorthand syntax is allowed
    let w = first_word_slices_everywhere(&s);
    println!("{}", w.len());
    println!("{w}");

    // Slices are general and apply to all collections (like arrays)
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];
    for i in slice.iter() {
        println!("{i}");
    }
}
