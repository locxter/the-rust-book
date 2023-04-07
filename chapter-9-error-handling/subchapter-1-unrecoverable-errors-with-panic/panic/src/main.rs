fn main() {
    // Rust differentiates between recoverable errors such as a missing file and unrecoverable ones that are caused by
    // bugs Unrecoverable errors can be handeled with the panic! macro, which throws a error to the console and then
    // closes the program by panicing println!("Some very complicated code here...");
    // panic!("Crash and burn!");
    // If the panic is called by external code, you can use the backtrace functionality to find its cause by running the
    // project with "RUST_BACKTRACE=1 cargo run"
    let vec = vec![1, 2, 3];
    println!("{}", vec[314]);
    // Obviously, recovering from errors is much nicer though...
}
