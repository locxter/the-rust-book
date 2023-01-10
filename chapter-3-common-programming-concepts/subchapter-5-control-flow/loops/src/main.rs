fn main() {
    // Endless loop only breakable with "break"
    let mut i = 0;
    loop {
        if i == 100 {
            break;
        }
        println!("Hello, world! {i}");
        i += 1;
    }

    // Return value of loops
    i = 0;
    let result = loop {
        if i == 10 {
            break i * 2;
        }
        i += 1;
    };
    println!("The result is: {result}");

    // Using loop labels to brake higher level loops
    let mut count = 0;
    'counting_up: loop {
        println!("Count is: {count}");
        let mut remaining = 10;
        loop {
            println!("Remaining is: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count is: {count}");

    // Conditional loops with while
    i = 10;
    while i > 0 {
        println!("{i}!");
        i -= 1;
    }
    println!("LIFTOFF!!!");

    // Even simpler with for loops
    for j in 0..10 {
        println!("{j}!");
    }
    println!("BACKWARDS LIFTOFF!!!");
    for j in (0..10).rev() {
        println!("{j}!");
    }
    println!("FORWARDS LIFTOFF!!!");

    // Looping over a collection with for
    let arr = [11, 22, 33, 44, 55];
    for num in arr {
        println!("The value is: {num}");
    }
}
