// Our first library crate with a module that has submodules for better structured code
mod front_of_house {
    // A submodule
    mod hosting {
        // A function within crate::front_of_house::hosting
        fn _add_to_waitlist() {}

        // Another function in the same module and therefore a sibbling of "add_to_waitlist"
        fn _seat_at_table() {}
    }

    // Another submodule and the sibbling of "hosting"
    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}
