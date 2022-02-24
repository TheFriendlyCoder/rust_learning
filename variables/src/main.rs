fn immutable() {
    let x = 5;
    println!("The value of x is {}", x);
    // This line triggers an error
    // can't reassign a value to an immutable variable
    // variables are immutable by default
    // x = 6;
    println!("The value of x is now {}", x);
}

fn mutable() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is now {}", x);
}

fn constants() {
    // declaring a constant without an explicit type
    // annotation is invalid
    //const X = 10;

    // you can assign a constant any value that is
    // known at compile time for some reason
    const Y: u32 = 9 * 4;
    println!("Value of 9 x 4 is {}", Y);
}

fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("X value in the inner scope is {}", x);
    }

    println!("X value in the outer scope is {}", x);
}
fn main() {
    immutable();
    mutable();
    constants();
    shadowing();
}
