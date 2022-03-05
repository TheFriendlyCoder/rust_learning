use crate::back_of_house::Appetizer;
mod front_of_house;

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub fn sample1() {
        let mut meal = Breakfast {
            toast: String::from("white"),
            // TBD: why is it that we can construct this struct here and
            // access the private seasonal_fruit member, but we can't from
            // the parent modle?
            seasonal_fruit: String::from("watermelon"),
        };
        println!("My default meal is {:?}", meal);

        // TBD: why can I access the private property seasonal_fruit here?
        println!("Seasonal fruit is {}", meal.seasonal_fruit);

        // The private/public identifiers only apply to visibility it seems
        // So long as you can see the private property, you can change it
        meal.seasonal_fruit = String::from("banana");
    }

    mod fubar {
        fn fix_incorrect_order() {
            println!("Ooooops. Fixing order...");
            cook_order();
            // Calls the serve_order() method from the back_of_house module
            super::serve_order();

            // Calls the serve_order() method from the crate root
            super::super::serve_order();
        }

        fn cook_order() {
            println!("Cooking new order...");
        }
    }

    fn serve_order() {
        println!("Custom serve order...");
    }
}

fn serve_order() {
    println!("Serving order...");
}

pub fn eat_at_restaurant() {
    // Absolute path to function...
    // "create" module is implicitly defined as module root
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path to function...
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please!", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");

    // For some reason we can't construct an instance of the Breakfast
    // struct from this scope but we can from within the back_of_house
    // module. Why?
    // let meal2 = back_of_house::Breakfast {
    //     toast: String::from("white"),
    //     seasonal_fruit: String::from("strawberries"),
    // };
    // println!("My second meal is {:?}", meal2);
    back_of_house::sample1();
    front_of_house::hosting::add_to_waitlist();

    // NOTE: we can reference enum variants so long as the enum is
    // marked as public, which contradicts the way structs work
    // TBD: find out why we can reference the Appetizer enum here without
    //      providing the full path to the enum
    //      More interestingly, how does this differentiate between the
    //      front_of_house::Appetizer and the back_of_house::Appetizer, both
    //      enums with the same names but different variants?
    // Answer: after I put code that referenced the Appetizer enum, VSCODE detected
    //         there was only one valid name in scope that looked like that, so it
    //         automatically added a 'use' statement to the top of my file, making
    //         the name directly accessible here!
    let appetizer = Appetizer::Salad;
    let appetizer2 = front_of_house::Appetizer::Bread;
}
