// Our first trait describing a .summarize method in an abstract way
pub trait Summary {
    fn summarize(&self) -> String;
    // Alternatively, we can also provide a default implementation
    // fn summarize(&self) -> String {
    //     String::from("(Read more...)")
    // }
    // Inside of which we could also use other trait methods like .summarize_author if it existed
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Our first trait-based implementation to give the abstract .summarize method a concrete body
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
// Alternatively, we could also use the default implementation
// impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Another trait-based implementation to give the abstract .summarize method a concrete body
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
