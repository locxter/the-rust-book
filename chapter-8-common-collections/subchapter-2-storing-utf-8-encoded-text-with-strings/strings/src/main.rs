fn main() {
    // Besides the commonly used string slice type (&str) for string literals and borrows, Rust also offers a truely mutuable and UTF-8 encoded String type
    // Strings borrow many aspects from vector like their creation
    let mut _str = String::new();
    // Alternatively, we can also start with other data
    let literal = "Some string literal";
    _str = literal.to_string();
    _str = String::from(literal);
    _str = "Even this works".to_string();
    _str = (5 * 5).to_string();
    // The big advantage of strings in Rust is their UTF-8 encoding, which allows fancy things like this without any additional code
    let greetings = [
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שָׁלוֹם"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];
    for message in greetings {
        println!("{message}");
    }
    // You can add a string slice using .push_str(), a single character using .push() or simply use the + operator
    let mut hello_world = String::from("Hello");
    hello_world.push(' ');
    hello_world.push_str("worl");
    hello_world += "d!";
    println!("{hello_world}");
    // You can also combine strings with it, but keep in mind that it takes ownership by default (str1 can not be used afterwards)
    let str1 = String::from("tic");
    let str2 = String::from("tac");
    let str3 = String::from("toe");
    let str4 = str1 + "-" + &str2 + "-" + &str3;
    println!("{str4}");
    // There also is the format! macro, which works like println!, but returns a String instead of printing the contents
    let str5 = format!("str2: {str2}\nstr3: {str3}\nAnd so on...");
    println!("{str5}");
    // Simple indexing to get a letter by doing &str[0] is not allowed though, as due to the UTF-8 encoding some characters take up more than 1 byte of memory:
    // 5 characters that use 5 bytes of memory
    let _english_greeting = "hello";
    // 12 characters that use 24 bytes of memory
    let other_greeting = "Здравствуйте";
    // If you really want to mess the with underlying data, you can get string slices though (might crash your program)
    let slice = &other_greeting[0..4];
    println!("{slice}");
    // Ideally, you just tell Rust what string representation you want with .chars() or .bytes()
    for char in other_greeting.chars() {
        println!("{char}");
    }
    for byte in other_greeting.bytes() {
        println!("{byte}");
    }
}
