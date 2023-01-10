// My first function in Rust ;)
fn some_function() {
    println!("Some function!");
}

// Some functions using parameters
fn print_x(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

// A function using an internal expression
fn some_func_using_an_expression() {
    let y = {
        let x = 3;
        // No semicolon here as it is an expression
        x + 1
    };
    println!("The value of y is: {y}");
}

// Return types are specified with "->" and functions are also expressions
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Use of "return" is also possible, but it is mostly used for early returns
fn early_return(x: i32) -> i32 {
    if x < 0 {
        return -1;
    }
    x
}

fn main() {
    println!("Hello, world!");
    some_function();
    print_x(5);
    print_labeled_measurement(5, 'h');
    some_func_using_an_expression();
    print_x(five());
    print_x(plus_one(5));
    print_x(early_return(5));
    print_x(early_return(-2));
}
