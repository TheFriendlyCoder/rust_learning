// NOTE: the name of the module is now defined by the file name
//       of this file (ie: Rust sets it implicitly) so we can't
//      redefine it again explicitly in the code like this
//mod front_of_house {
pub enum Appetizer {
    Yogurt,
    Bread,
}

// TODO: see if it is possible to define an entire module within a subfolder, or
//       if you always need to have a Rust rs file of the same name defined.

// Re-export the "hosting" module, which is a child module under
// the "front_of_house" module. In this case the sub-module is defined
// in a file named "hosting" within a folder called "front_of_house" and
// the parent-child relationship is implicit based on Rusts rules for
// module loading
// NOTE: this needs to be 'pub' so that users of the front_of_house module
//      can see and access it without having to use / import it directly
pub mod hosting;

mod serving {
    fn take_order() {
        println!("Taking customer order...");
        //super::hosting::add_to_waitlist();
    }

    fn serve_order() {
        println!("Serving order to customer...");
    }

    fn take_payment() {
        println!("Making Money!!!!");
    }
}
//}
