fn infinite_loop() {
    // user must kill the application to terminate this loop
    loop {
        println!("Hello, world!");
    }
}

fn loop_labels() {
    'outer_loop: loop {
        println!("Starting outer loop");
        loop {
            println!("Inner loop starting");
            // Using this break, the inner loop will exit but pass control
            // back to the outer loop on the line immediately following this
            // loop block
            break;

            // Using this break, the outer loop is exited and control
            // passes back up to the parent function
            break 'outer_loop;

            println!("Should not see me");
        }
        //This next line will execute if we only 'break' from the previous loop
        // but if we 'break' using the outer_loop label then this line will not
        // get executed and control passes back to the parent function
        println!("Ending loop");
        break;
    }

    // this line should always execute regardless of which break statement is used
    println!("Finishing function outside a loop");
}

fn loop_expressions() {
    let mut counter = 0;

    let result = loop {
        counter = counter + 1;
        if counter >= 10 {
            // we can provide a value to the 'break' statement to pass a value back
            // to the calling code, making this loop block an 'expression'
            // useful when retrying an operation several times and returning the
            // final result
            break counter * 5;
        }
    };

    println!("The value of result is {}", result);
}

fn while_example() {
    let mut counter = 10;
    while counter > 5 {
        counter -= 1;
        println!("The value of counter is now {}", counter);
    }

    println!("Final vaue for counter is {}", counter);
}

fn for_example() {
    let a = [5, 10, 15, 20, 25];

    for element in a {
        println!("Current element is {}", element);
    }
}

fn for_range() {
    // 1..5 defines a range of number from 1 to 5 excluding 5 (ie: 1, 2, 3, 4)
    // 1..=5 defines a range of numbers from 1 to 5 inclusive (ie: 1, 2, 3, 4, 5)
    // ().rev() reverses the order of a range
    for element in (1..=5).rev() {
        println!("Counting down... {}", element);
    }
}
fn main() {
    loop_labels();
    loop_expressions();
    while_example();
    for_example();
    for_range();
}
