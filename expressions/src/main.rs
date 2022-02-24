fn main() {
    // This is a statement because it performs an operation and returns no value
    println!("Hello, world!");

    // 4 * 3 is an expression because it returns a value, and that return value
    // is them bound to the label 'x' as part of the assignment operator in
    // this statement (ie: this statement contains an expression)
    let _x = 4 * 3;

    // scope blocks, delimited by curly braces, are expressions. They return
    // values. In this case the return value from the scope block will be
    // 15 (ie: because the last statement in the block doesn't end with a semicolon
    // the value of the expression is returned from the scope block)
    let y = {
        let z = 14;
        z + 5
    };
    println!("The value of y is {}", y);

    // NOTE: the default return value from an expression is the "unit value" ()
    // Also note that we need to use the debug format string to show the unit
    // value because it doesn't implement the string conversion / display interace
    let y2 = {};
    println!("The value of empty expression is {:?}", y2);
}
