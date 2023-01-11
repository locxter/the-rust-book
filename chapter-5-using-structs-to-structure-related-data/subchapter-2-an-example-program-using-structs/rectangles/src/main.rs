// A simple, but helpful rectangle struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// A dead simple rectangle area function
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Another dead simple rectangle area function, but less readable as the fields are not named
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Finally a readable and descriptive rectangle area function
fn area_rect(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    // Unrelated values work, but are just bad
    let width1 = 30;
    let height1 = 50;
    println!("Area: {}", area(width1, height1));
    // Using a tuple makes things somewhat better
    let rect1 = (30, 50);
    println!("Area: {}", area_tuple(rect1));
    // But only structs are the way to go here
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area: {}", area_rect(&rect2));
    // Printing an instance of our struct will not work as does not implement any formatting by default
    // println!("rect2: {}", rect2);
    // For debug prints we only have to add "#[derive(Debug)]" to the struct and use debug formatting with ":?" (or ":#?" for pretty output)
    println!("rect2: {:?}", rect2);
    println!("rect2: {:#?}", rect2);
    // And for really fast debugging a marco exists for this (prints to stderr though)
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect3);
    // Only if the area function would be bounded to our struct...
}
