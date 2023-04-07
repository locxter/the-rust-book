use std::net::IpAddr;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, but is {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // Panicing is always a good solution for examples, prototyping and tests as they might become harder to understand,
    // take longer or should fail in total if some part fails Additonally, panicing is fine in cases you can ensure
    // a Result will always have an Ok value like here:
    let _localhost: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");
    // Furthermore, it is advisable to have your code panic when it is possible that your code could end up in a bad
    // state that is not expected However, when failure is expected, it is more appropriate to return a Result than
    // to make a panic! call. It is also generally advisible to use the type system of Rust (potentially with custom
    // types) to ensure the validity of data
    let my_guess = Guess::new(50);
    let someones_guess = Guess::new(200);
    println!("My guess: {}", my_guess.value());
    println!("Someones guess: {}", someones_guess.value());
}
