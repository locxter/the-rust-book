// Modules and functions are private to their parents by default
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    // Enums are allowed to be public as well (with public fields by default)
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // Structs can also be made public, but their fields stay private by default
    #[allow(dead_code)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // The same applies to the functions inside of an implementation
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    pub fn fix_incorrect_order() {
        cook_order();
        // The super keyword allows you to go up the module tree in relative paths
        super::deliver_order();
    }

    fn cook_order() {}
}

// Public function using parts of private modules
pub fn eat_at_restaurant() {
    // Absolute path (preferred)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    back_of_house::fix_incorrect_order();

    // Enum fields are public by default
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    // The toast field can be accessed and changed just fine as it is public
    let mut meal = back_of_house::Breakfast::summer("Rye");
    println!("Toast: {}", meal.toast);
    meal.toast = String::from("Wheat");
    println!("Toast: {}", meal.toast);
    // But same would fail for the private seasonal_fruit field
    // println!("Seasonal fruit: {}", meal.seasonal_fruit);
    // meal.seasonal_fruit = String::from("Banana");
    // println!("Seasonal fruit:: {}", meal.seasonal_fruit);

    // Basically, things (except enums) are private by default - unless annotated with the pub keyword
}

fn deliver_order() {}
