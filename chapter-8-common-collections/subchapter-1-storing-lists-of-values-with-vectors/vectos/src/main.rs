// Our hacky, multi-type enum
enum MultiType {
    Int(i32),
    Float(f64),
    String(String),
}

fn main() {
    // Vectors are basically resizable arrays, which are therefore stored in the heap
    let _vec: Vec<i32> = Vec::new();
    // There is a macro with creating a vector with initial values
    let mut vec = vec![1, 2, 3];
    // More values can be added with the .push() method if the vector is mutual
    vec.push(4);
    vec.push(5);
    // Values can be either read using the unsafe array bracket syntax (panics if the requested elements is outside the
    // vector bounds) or using the safe .get() method (returns Option<&T>, which needs to be handeled safely)
    let third = &vec[2];
    println!("{third}");
    let fourth = vec.get(3);
    match fourth {
        Some(fourth) => println!("{fourth}"),
        None => println!("Element does not exist"),
    }
    // The ownership and borrowing rules still apply, so this code will not run as is uses immutable and mutuable
    // references at the same time let first = &vec[0];
    // vec.push(6);
    // println!("{first}");
    // Here is how to iterate over all elements using a for loop
    for num in &vec {
        println!("{num}");
    }
    // Alternatively, we can also use a mutuable reference to change the values
    for num in &mut vec {
        *num += 10;
        println!("{num}");
    }
    // A vector can only store values of the same type, but enums allow us to hack around this limitation
    let _another_vec = vec![
        MultiType::Int(3),
        MultiType::Float(3.142),
        MultiType::String(String::from("Pi")),
    ];
    // As always, vectors are scope limited by design
}
