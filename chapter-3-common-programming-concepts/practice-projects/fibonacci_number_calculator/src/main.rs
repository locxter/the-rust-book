use std::io;

fn main() {
    println!("Fibonacci number calculator");
    loop {
        let n: i128 = loop {
            let mut count = String::new();
            println!("\nEnter n:");
            io::stdin().read_line(&mut count).expect("Failed to read line");
            break match count.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        };
        if n <= 0 || n >= 186 {
            break;
        }
        println!("\nn Fibonacci numbers:");
        if n == 1 {
            println!("0");
        } else {
            let mut a: i128 = 0;
            let mut b: i128 = 1;
            let mut c: i128;
            println!("0\n1");
            for _ in 2..n {
                c = a + b;
                a = b;
                b = c;
                println!("{b}");
            }
        }
    }
}
