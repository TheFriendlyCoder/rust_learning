fn string_mutability() {
    let mut s = String::from("Hello");

    // unlike string literals, String objects can be changed at runtime
    s.push_str(" World!!!");

    println!("Value of s is '{}'", s);
}

fn string_ownership() {
    let first = String::from("John Doe");
    let second = first;

    println!("The value of second is '{}'", second);
    // This next line fails because "first" no longer "owns" the reference to the String object
    // that was transferred to "second" during assignment on the second line
    //println!("The value of first is '{}'", first);
}

fn string_cloning() {
    let first = String::from("John Doe");
    let mut second = first.clone();

    println!("The value of second is '{}'", second);
    println!("The value of first is '{}'", first);

    second.push_str(" was here");
    println!("Second is now '{}'", second);
    // you will see that the contents of the original String
    // is still the original "John Doe" string because "second"
    // copied the heap data for the string
    println!("First is now '{}'", first);
}

fn takes_ownership(data: String) {
    println!("The owned string is {}", data);
}

fn makes_copy(data: i32) {
    println!("The copied data is {}", data);
}

fn show_ownership() {
    let s = String::from("Take my ownership");
    takes_ownership(s);
    // This next line fails because this function no longer owns the String
    // object, and once the 's' value was passed to 'takes_ownership' it became
    // invalidated once the method finished and the passed argument went
    // out of scope
    //println!("Value of owned string is {}", s);

    let x = 15;
    makes_copy(x);
    println!("Value of X is still valid: {}", x);
}

fn gives_ownership() -> String {
    let some_string = String::from("My passed string");
    some_string
}

fn takes_and_gives_ownership(mut data: String) -> String {
    data.push_str(" is modified");
    data
}

fn test_return_values() {
    let x = gives_ownership();
    println!("Value of X is {}", x);

    let y = takes_and_gives_ownership(x);
    println!("Value of Y is {}", y);

    // this next line fails because 'x' transfered ownership to the
    // 'takes_and_gives_ownership' when passed as a parameter
    //println!("Final value of X is {}", x);
}
fn main() {
    string_mutability();
    string_ownership();
    string_cloning();
    show_ownership();
    test_return_values();
    // NOTE: POD types are always "deep copied" so they have no "clone" method
    //      so you need to be aware of the types of data when reading code to
    //      understand the copy semantics
}
