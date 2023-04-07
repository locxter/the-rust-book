enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum UsState {
    _Alabama,
    Alaska,
}

enum SpecialCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn to_cents(&self) -> u8 {
        // The match construct compares a value against all possible patterns and executes the code of the arm where the
        // pattern matches
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

impl SpecialCoin {
    fn to_cents(&self) -> u8 {
        // The match construct can also bind to internal parts of the value that matches the pattern
        match self {
            SpecialCoin::Penny => 1,
            SpecialCoin::Nickel => 5,
            SpecialCoin::Dime => 10,
            SpecialCoin::Quarter(state) => {
                println!("State: {:?}", state);
                25
            }
        }
    }
}

// A safe x + 1 function using the Option enum and match construct
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    for coin in coins {
        println!("{}", coin.to_cents());
    }
    let special_coins = [
        SpecialCoin::Penny,
        SpecialCoin::Nickel,
        SpecialCoin::Dime,
        SpecialCoin::Quarter(UsState::Alaska),
    ];
    for special_coin in special_coins {
        println!("{}", special_coin.to_cents());
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);
    // By default the last arm covers every possible pattern that is not explicitly mentioned
    let dice_roll = 9;
    match dice_roll {
        3 => println!("You get a fancy hat!"),
        7 => println!("You loose your fancy hat!"),
        other => println!("You move {} spaces", other),
    }
    // If we do not care about the value in the catch-all pattern, we can use _ to throw it away
    match dice_roll {
        3 => println!("You get a fancy hat!"),
        7 => println!("You loose your fancy hat!"),
        _ => println!("You move a space"),
    }
    // And if we do not want to run code in the catch-all arm altogether, we can write ()
    match dice_roll {
        3 => println!("You get a fancy hat!"),
        7 => println!("You loose your fancy hat!"),
        _ => (),
    }
}
