fn main() {
    // Does not work because the value changes and needs to be mutable
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 3;
    println!("The value of x is: {x}");
    // Does not work because constants need to be evaluated at compile time
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * x;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
    // Shadowing to declare a new variable (possibly with another data type) with the same name as a previous variable
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The user wants this many spaces: {spaces}");
}
