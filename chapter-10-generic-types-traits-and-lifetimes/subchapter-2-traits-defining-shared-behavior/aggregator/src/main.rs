use aggregator::{Summary, Tweet};

// Our first trait-driven function using the impl syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// If our parameter needs to implement multiple traits, we could write:
// pub fn notify(item: &(impl Summary + Display)) {

// Our first trait-driven function using the full trait bound syntax
pub fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// If our parameter needs to implement multiple traits, we could alternatively write:
// pub fn notify_bound<T: Summary + Display>(item: &T) {

// Instead of writing all of our trait bounds like this:
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// We can write them in an easier to read way using the where syntax like this:
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

fn main() {
    // Now that we know generics, it is time to look at how to define their behavior in an abstract manner (somewhat
    // like interfaces in other languages) Traits allow us to group related method signatures together and apply
    // them to a certain set of types, see lib.rs
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    // Of course, traits can be used to define the data type of parameters in a generic, but exact way
    notify(&tweet);
    notify_bound(&tweet);
    // And are also valid for the return type of functions using the -> impl Summary { syntax
    // Finally, we already looked at how to use trait bounds to conditionally implement methods in the last chapter with
    // the impl<T: PartialOrd> Pair<T> { syntax
}
