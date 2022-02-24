fn rounding() {
    let mut x: u8 = 254;
    x = x + 1;
    println!("Value of X is {}", x);

    // When run in debug the following line triggers an error
    // When run in release the following lines work and produce "Value of X is 0"
    // x = x + 1;
    // println!("Value of X is {}", x);
}

fn characters() {
    // Characters are defined by single quotes
    // double quotes delineate multi character strings
    // characters are always 4-byte Unicode point points
    // simple ASCII characters can be decoded as integer values  as well (ie: 'z' is ASCII 122)
    let char = 'z';
    println!("The ascii value for {} is {}", char, char as u32);
}

fn tuple_deconstruction() {
    // tuples may contain different types
    let my_tuple = (5i16, 10u8, 15i32);

    // here, x = 5i16, y = 10u8, and z = 15i32
    let (x, y, z) = my_tuple;

    // Raises an error in debug because it causes an overflow of a u8
    // but succeeds in release because of overflow rounding
    //let _w = y + 250;

    println!("X {}, Y {}, Z {}", x, y, z);

    // Can use . notation to refer to tuple element by index number
    println!("10 + 4 is {}", my_tuple.1 + 4);

    // special single-element type called the "unit type" which has a value called the "unit value"
    // can't be displayed with normal printing, have to use debug formatting
    let empty_tuple = ();
    println!("Empty tuple is {:?}", empty_tuple);
}
fn main() {
    rounding();
    characters();
    tuple_deconstruction();
}
