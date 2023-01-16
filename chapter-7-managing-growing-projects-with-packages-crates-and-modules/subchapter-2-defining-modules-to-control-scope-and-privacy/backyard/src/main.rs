// Bring a module into scope to reduce the path length
use crate::garden::vegetables::Asparagus;

// Create a module and make it public with pub
pub mod garden;

fn main() {
    // Thanks to "use" no long paths are needed
    let plant = Asparagus {};
    println!("My plant: {:?}", plant);
}
