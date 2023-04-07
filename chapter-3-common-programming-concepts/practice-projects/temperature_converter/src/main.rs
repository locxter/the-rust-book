use std::io;

fn main() {
    println!("Temperature converter");
    loop {
        let mode: i32 = loop {
            let mut mode = String::new();
            println!("\nSelect the conversion mode:");
            println!("1: Celsius to Fahrenheit");
            println!("2: Fahrenheit to Celsius");
            println!("3: Quit");
            io::stdin().read_line(&mut mode).expect("Failed to read line");
            break match mode.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        };
        if mode <= 0 || mode >= 3 {
            break;
        }
        let input_temp: i32 = loop {
            let mut input_temp = String::new();
            println!("\nEnter the temperature to convert:");
            io::stdin().read_line(&mut input_temp).expect("Failed to read line");
            break match input_temp.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        };
        if mode == 1 {
            println!("\nTemperature in Celsius: {input_temp}");
            println!(
                "Temperature in Fahrenheit: {}",
                (input_temp as f64 * (9.0 / 5.0)) + 32.0
            );
        } else {
            println!("\nTemperature in Fahrenheit: {input_temp}");
            println!("Temperature in Celsius: {}", (input_temp - 32) as f64 * (5.0 / 9.0));
            println!("Temperature in Celsius: {}", (input_temp - 32) as f64 * (5.0 / 9.0));
        }
    }
}
