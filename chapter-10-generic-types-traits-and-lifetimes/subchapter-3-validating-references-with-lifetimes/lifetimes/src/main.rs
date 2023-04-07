use std::fmt::Display;

// Our first function using the lifetime annotation syntax to make sure that our returned reference is valid as long as
// both parameters are valid
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Our first struct using the lifetime annotation syntax
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// A function exploiting the first two lifetime elision rules
// fn first_word<'a>(s: &'a str) -> &'a str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Our first implementation exploiting all three lifetime elision rules
#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    // Rules one and two
    fn level(&self) -> i32 {
        3
    }

    // Rules one and three
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Our first function combining generic type parameters, trait bounds, and lifetimes
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Lifetimes are another kind of generics that we have already been using. Rather than ensuring that a type has the
    // behavior we want, lifetimes ensure that references are valid as long as we need them to be. Here is an
    // example to explain this concept:
    let _r;
    {
        let x = 5;
        _r = &x;
    }
    // println!("r: {r}");
    // This does not work as x goes out of scope before we reference it here and Rust checks for these errors at compile
    // time using a borrow checker ('b is smaller than 'a) In order to mitigate these issues Rust provides a
    // lifetime annotation syntax - check the function at the top
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_str(string1.as_str(), string2);
    println!("Longest string: {}", result);
    // Therefore, this will not work as string3 goes out of scope before we use other_result:
    let _other_result;
    {
        let string3 = String::from("efghi");
        _other_result = longest_str(string1.as_str(), string3.as_str());
    }
    // println!("The longest string is {}", other_result);
    // This syntax pretty also applies to structs
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt { part: first_sentence };
    // But Rust also features a few so-called lifetime elision rules that make our life easier by creating lifetime
    // annotation automatically in a few deterministic cases. The first rule is that the compiler assigns a lifetime
    // parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime
    // parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn
    // foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on. The second rule is that, if there is exactly one input
    // lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a
    // i32. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or
    // &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This
    // third rule makes methods much nicer to read and write because fewer symbols are necessary. If the compiler
    // can get to the end of these rules without dangling references, we do not have a specifiy them manually. See the
    // first_word function and ImportantExcerpt implementation for an examples of this:
    let str = "Hello world!";
    println!("First word: {}", first_word(str));
    println!("Excerpt level: {}", excerpt.level());
    println!("Excerpt: {}", excerpt.announce_and_return_part("Some excerpt!"));
    // There is also the special 'static lifetime, which denotes that the affected reference can live for the entire
    // duration of the program. But before specifying 'static as the lifetime for a reference, think about whether
    // the reference you have actually lives the entire lifetime of your program or not, and whether you want it to.
    // Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling
    // reference or a mismatch of the available lifetimes. In such cases, the solution is fixing those problems, not
    // specifying the 'static lifetime.
    let _str: &'static str = "String literals always have a static lifetime as they are stored in the binary.";
    // Finally, here is a function combining generic type parameters, trait bounds, and lifetimes:
    println!(
        "Longest string: {}",
        longest_with_an_announcement(string1.as_str(), string2, "Hello world!")
    );
}
