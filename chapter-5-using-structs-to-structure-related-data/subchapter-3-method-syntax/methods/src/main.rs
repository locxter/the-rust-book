// A simple, but helpful rectangle struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Our first implementation of methods and associated functions
impl Rectangle {
    // Our beloved area function now as an method
    fn area(&self /* or self: &Self */) -> u32 {
        self.width * self.height
    }

    // Methods can use the same name as one of the struct fields
    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }

    // The usage of multiple parameters and other instances of the same struct as such is also valid
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }

    // So called associated functions that are part of an implemenation but do not use the self reference are also possible (especially useful for constructors)
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn new_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Methods also use the dot notation and therefore are easy to understand
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area: {}", rect1.area());
    println!("Nonzero width: {}", rect1.width());
    println!("Nonzero height: {}", rect1.height());
    // The access to struct fields remains the same
    println!("Width: {}", rect1.width);
    println!("Height: {}", rect1.height);
    // Here some easy to read comparisions are going on
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
    // Our associated functions in use (called with :: instead of .)
    let rect4 = Rectangle::new(30, 50);
    let square = Rectangle::new_square(50);
    println!("Area: {}", rect4.area());
    println!("Area: {}", square.area());
    // You can also split your methods and associated functions into any number of implemntation blocks, which can be useful sometimes
}
