// The use keyword can save us a lot of writing (scope limited)
use crate::front_of_house::hosting;
use std::collections::HashMap;
// Conflicts have to be avoided, so this is illegal
// use std::fmt::Result;
// use std::io::Result;
// Instead just specify their parents and explicitly select one (see function1 and function2)
// use std::fmt;
// use std::io;
// Alternatively, you can also rename one or both of them
// use std::fmt::Result as FmtResult;
// use std::io::Result as IoResult;
// We can also make use statements accessible to outside code with pub
// pub use crate::front_of_house::hosting;
// As always a few shortcuts are available
// "use std::cmp::Ordering; use std::io;"" is the same as "use std::{cmp::Ordering, io};"
// "use std::io; use std::io::Write;" is the same as "use std::io::{self, Write};"
// And there is a wildcard to import everything "use std::collections::*;"

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Shortened path thanks to use
    hosting::add_to_waitlist();
    // You could also add ::add_to_waitlist to the use statement, but that is bad style

    // Nevertheless, you do this for structs, enums and so on (especially from the standard library)
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// fn function1() -> fmt::Result {}

// fn function2() -> io::Result<()> {}
