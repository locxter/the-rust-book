fn main() {
    // Using the match construct is a bit verbose if we only care about a single case
    let config_max = Some(3);
    match config_max {
        Some(max) => println!("Max: {max}"),
        _ => {}
    }
    // So Rust has the if let construct to make things more consise
    if let Some(max) = config_max {
        println!("Max: {max}");
    }
    // It even supports an else part which is identical to the _ pattern
    if let Some(max) = config_max {
        println!("Max: {max}");
    } else {
        println!("Something went wrong!")
    }
    // Bonus: You can use match or if let to extract data inside an enum into another variable
    let match_max = match config_max {
        Some(max) => max,
        _ => 0,
    };
    let if_let_max = if let Some(max) = config_max { max } else { 0 };
    println!("{match_max}");
    println!("{if_let_max}");
}
