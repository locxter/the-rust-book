use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

// Our first function propagating internal errors to the calling code, which can then decide what to do (panic, try
// again or use a default value)
fn _read_username() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

// Since propagating errors is so common, there is the ? operator for it
fn _read_another_username() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// These operators can also be changed for even cleaner code
fn _read_yet_another_username() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// And since reading a file is pretty common, there even is a standard library function for it
fn _read_username_lazy() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// The ? operator also works with the Option<T> type (None instead of Err and Some instead of Ok)
fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    // Most errors are not serious enough to require a full programm stop and therefore can be handeled with the
    // Result<T, E> type, which can be either Ok or Err and includes the actual result of an operation
    let greeting_file_result = File::open("hello.txt");
    // We can handle this easily with a match statement
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
    // Or even better, we can chain multiple match statements together to handle different errors in unique ways
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            },
            error => panic!("Problem opening the file: {:?}", error),
        },
    };
    // If we do not care about panicing, we can simply get to the actual operation result with .unwrap()
    let _greeting_file = File::open("hello.txt").unwrap();
    // Alternatively, .expect() does the same thing but allows us to provide our own error message
    let _greeting_file = File::open("hello.txt").expect("The file 'hello.txt' does not exist");
    // Functions can also pass their internal errors on to external code, look above
    // This can also be done in the main function by settings its return type to Result<(), Box<dyn Error>> and adding
    // Ok(()) at the end
}
