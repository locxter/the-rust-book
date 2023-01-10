fn main() {
    // Does not work because the compiler needs more information about the data type
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("You guessed: {guess}");

    // Integers (i8, i16, i32, i64, i128, u8, u16, u32, u64, u128)
    let num = 5;
    println!("Num is: {num}");
    let x: i64 = 3596;
    println!("X is: {x}");
    // Use a type suffix to describe the exact data type
    let y = 57u8;
    println!("Y is: {y}");
    // Use "_" as a visual separator with any meaning
    let z = 7_596_384;
    println!("Z is: {z}");
    // Integer literals
    let dec = 98_222;
    println!("Dec is: {dec}");
    let hex = 0xff;
    println!("Hex is: {hex}");
    let oct = 0o77;
    println!("Oct is: {oct}");
    let bin = 0b1111_0000;
    println!("Bin is: {bin}");
    let byt = b'A';
    println!("Byt is: {byt}");

    // Floating-point numbers (f32, f64)
    let num = 2.1;
    println!("Num is: {num}");
    let x: f32 = 3.142;
    println!("X is: {x}");

    // Numeric operations
    // Addition
    let sum = 5 + 10;
    println!("Sum is: {sum}");
    // Subtraction
    let difference = 95.5 - 4.3;
    println!("Difference is: {difference}");
    // Multiplication
    let product = 4 * 30;
    println!("Product is: {product}");
    // Division
    let quotient = 56.7 / 32.2;
    println!("Quotient is: {quotient}");
    let integer_quotient = -5 / 3;
    println!("Integer quotient is: {integer_quotient}");
    // Remainder
    let remainder = 43 % 5;
    println!("Remainder is: {remainder}");

    // Booleans (bool)
    let t = true;
    println!("T is: {t}");
    let f: bool = false;
    println!("F is: {f}");

    // Characters (char)
    let c = 'z';
    println!("C is: {c}");
    // Unicode compatibility
    let z: char = 'ℤ';
    println!("Z is: {z}");
    let heart_eyed_cat = '😻';
    println!("Heart eyed cat is: {heart_eyed_cat}");

    // Tuples for grouping together a number of values with a variety of types
    let first_tup = (500, 6.4, 1);
    let mut second_tup: (u16, f32, u8) = (465, 3.4, 8);
    // Print them out
    println!("First tup is: {:?}", first_tup);
    println!("Second tup is: {:?}", second_tup);
    // Destruct into primitive values
    let (x, y, z) = first_tup;
    println!("Primitive values: {x}, {y}, {z}");
    // Access and change values
    second_tup.0 = 367;
    second_tup.1 = 3.67;
    second_tup.2 = 4;
    let x = second_tup.0;
    let y = second_tup.1;
    let z = second_tup.2;
    println!("Changed primitive values: {x}, {y}, {z}");

    // Arrays for groupung together values of the same type
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let mut months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // Initialize with all the same values;
    let seven = [7; 10];
    // Print them out
    println!("Arr is: {:?}", arr);
    println!("Months is: {:?}", months);
    println!("Seven is: {:?}", seven);
    // Destruct into primitive values
    let [a, b, c, d, e] = arr;
    println!("Primitive values: {a}, {b}, {c}, {d}, {e}");
    // Access and change values
    months[6] = "My birthday is in June";
    let jan = months[0];
    let feb = months[1];
    let mar = months[2];
    println!("Changed primitive values: {jan}, {feb}, {mar}");
}
