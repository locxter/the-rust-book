fn main() {
    // Simple if expressions
    let num = 3;
    if num < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    // Multiple conditions with else if (sometimes also possible with match)
    let num = 6;
    if num % 4 == 0 {
        println!("Number is divisible by 4");
    } else if num % 3 == 0 {
        println!("Number is divisible by 3");
    } else if num % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }

    // An if expression in a let statement (only possible with same types)
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("Num is: {num}")
}
