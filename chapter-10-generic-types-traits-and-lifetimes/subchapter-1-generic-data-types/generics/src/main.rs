// A simple function to find the largest number in an i32 array slice
fn largest_num(num_list: &[i32]) -> &i32 {
    let mut largest_num = &num_list[0];

    for num in num_list {
        if num > largest_num {
            largest_num = num;
        }
    }

    largest_num
}

// A generic function to find the largest value in any array slice (of a type that can be ordered)
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for value in list {
        if value > largest {
            largest = value;
        }
    }

    largest
}

// A simple point struct using generics and requiring all values have to be of the same type
#[allow(dead_code)]
struct UniformPoint<T> {
    x: T,
    y: T,
}

// An implementation that works with all possible types
impl<T> UniformPoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Another point struct using generics, but not requiring all values have to be of the same type
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// An implementation that only works with f64 types in all places
impl MixedPoint<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Another uniform point
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// A generic implementation mixing different generic types for better readibility
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: other.y }
    }
}

fn main() {
    // Functions already allow us to mitigate code duplication by writing it once and using it in multiple scenarios
    let num_list = vec![34, 50, 25, 100, 65];
    let result = largest_num(&num_list);
    println!("The largest number is {}", result);
    let num_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_num(&num_list);
    println!("The largest number is {}", result);
    // However, they are type-specific by default and this is where generics come in
    // Generics allow us to replace specific types with a placeholder that represents multiple types to remove even more
    // duplicate code
    let num_list = vec![34, 50, 25, 100, 65];
    let result = largest(&num_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    // Generics also work with structs, although you have to keep the syntax of generics in mind
    let integer_point = UniformPoint { x: 5, y: 10 };
    let float_point = UniformPoint { x: 1.0, y: 4.0 };
    // let mixed_point = UniformPoint { x: 5, y: 4.0 };
    let _mixed_point = MixedPoint { x: 5, y: 4.0 };
    let uniform_but_possibly_mixed_point = MixedPoint { x: 5.0, y: 4.0 };
    // The usage of generics is also possible with enums as we already learned when taking a look at Option<T> and
    // Result<T, E> And of cause you can also use generics in methods (and restrict their scope to certain data
    // types)
    println!("interger_point.x = {}", integer_point.x());
    println!("float_point.x = {}", float_point.x());
    println!(
        "uniform_but_possibly_mixed_point.distance_from_origin = {}",
        uniform_but_possibly_mixed_point.distance_from_origin()
    );
    // Generic type parameters in a struct definition do not always have to be the same as those you use in the struct’s
    // method signatures
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // Last, but certainly not least we have to note that using generics does not have any negative perfomance costs due
    // to Rust's clever compiler
}
