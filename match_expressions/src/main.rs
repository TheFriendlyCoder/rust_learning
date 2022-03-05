#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_in_cents(money: &Coin) -> u8 {
    match money {
        // NOTE: all of the "arms" in this match expression are
        //       expected to return their values back to the parent
        //       scope. This allows us to return the correct u8
        //       value back from this function without using a single
        //       return statement, simply by NOT putting an ending
        //       semicolon at the end of any of the arms.
        //
        // ie: Coin::Penny => {1;}    would not work because the ; defines / delimits a statement
        //                            you should get an "inconsistent data type" error because the
        //                            match expression expects all arms to return a u8 type
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn show_value() {
    let temp = Coin::Penny;
    println!("Value of {:?} is {}", temp, value_in_cents(&temp));
}

// NOTE: this enum needs to derive from the Debug trait so
//       the values can be printed, even though the Coin2
//       enum below also has the Debug trait defined. The
//       compiler will complain if you only use the latter because
//       it doesn't know how to render the data contained in the
//       Quarter "variant" of the Coin2 enum otherwise
#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    Texas,
    Florida,
}

#[derive(Debug)]
enum Coin2 {
    Penny,
    Nickle,
    Dime,
    Quarter(USState),
}

fn extract_enum_data() {
    //let money = Coin2::Quarter(USState::Florida);
    // other coin types don't need parameters passed to their constructors
    let money = Coin2::Penny;

    let value = match money {
        Coin2::Penny => 1,
        Coin2::Nickle => 5,
        Coin2::Dime => 10,
        // So here we are using a match expression for Quarter, but it has embedded
        // metadata. So we have to pass the empty placeholder "_" as a parameter to
        // tell Rust we know there is data there but we don't care about it
        Coin2::Quarter(state) => {
            // NOTE: curly braces needed for block expressions within
            //       a match arm
            println!("State on quarter is {:?}", state);
            // NOTE: final line in code block must not end with a semicolon
            //       in this case, so it is treated as an expression and its
            //       value can be returned from the match expression. The
            //       compiler will enforce this by making sure all match arms
            //       return the same data type
            25
        }
    };

    println!("Value of coin is {}", value);
}

fn ignore_enum_data() {
    let money = Coin2::Quarter(USState::Alaska);
    let value = match money {
        Coin2::Penny => 1,
        Coin2::Nickle => 5,
        Coin2::Dime => 10,
        // So here we are using a match expression for Quarter, but it has embedded
        // metadata. So we have to pass the empty placeholder "_" as a parameter to
        // tell Rust we know there is data there but we don't care about it
        Coin2::Quarter(_) => 25,
    };

    println!("Value of quarter is {}", value);
}

fn problem_case1() {
    //let money = Coin2::Quarter(USState::Florida);
    let money = Coin2::Penny;

    // printing our coin enumeration here is safe...
    println!("Coin type is {:?}", money);

    let value = match money {
        Coin2::Penny => 1,
        Coin2::Nickle => 5,
        Coin2::Dime => 10,

        Coin2::Quarter(state) => {
            println!("State on quarter is {:?}", state);
            25
        }
    };

    // printing the integer value for our value is safe (ie: no references to worry about)
    println!("Value of coin is {}", value);

    // BUT, attempting to print our enum data here causes a problem for some reason
    //println!("Coin type is {:?}", money);

    // NOTE: the problem with the above statement doesn't have anything to do with the
    //       nested println() statement in the match expression above. We get the same
    //       compiler error regardless

    // NOTE: The compiler has the same issue here whether the `money`
    // variable is of type Coin2::Quarter or not, meaning that it won't let you print
    // the value of money EVEN IF the match expression above doesn't actually result
    // in a transfer of ownership of data.
}

fn problem_case1_solution() {
    // Fix suggested by the compiler looks like this:

    let money = Coin2::Quarter(USState::Florida);
    //let money = Coin2::Penny;

    let value = match money {
        Coin2::Penny => 1,
        Coin2::Nickle => 5,
        Coin2::Dime => 10,

        // By adding the 'ref' keyword in the middle of our arm
        // expression we are telling Rust to borrow an immutable
        // reference to the data associated with the Quarter enum
        // (ie: &USState in this case), which then allows us to
        // continue using the "money" variable after the match
        // statement
        Coin2::Quarter(ref state) => {
            println!("State on quarter is {:?}", state);
            25
        }
    };

    println!("The value of {:?} is {}", money, value);
}

fn plus_one(i: Option<i32>) -> Option<i32> {
    match i {
        None => None,
        Some(data) => Some(data + 1),
    }
}

fn match_with_option_example() {
    let value1 = Some(52);
    let temp = plus_one(value1);

    println!(
        "The next value from {} is {}",
        value1.expect("fubar"),
        temp.expect("fubar")
    );

    let value2 = None;
    let temp = plus_one(value2);
    println!("Value of {:?} is {:?}", value2, temp);
}

fn catch_all_with_value() {
    //let value = Some("My value is here");
    let value = None;

    let res = match value {
        None => "Result was empty",
        // On this next line we use an arbitrary variable name on
        // the arm expression. This causes Rust to bind the original
        // match express (ie: "value" in this case) to this variable
        // so it can be used in the arm code block. So in this case
        // "data" will be of type Option<&str> so we use the expect()
        // method to extract the character string from the option, and
        // we basically ignore any errors because we know we always have
        // a valid value here (ie: because the None case is already
        // handled above)
        data => data.expect("fubar"),
    };

    println!("Result is {}", res);
}

fn catch_all_with_ignore() {
    let value = Some(42);

    let res = match value {
        None => 0,
        // Here we now the arm will match on a Some(i32) type but
        // we actually don't care about the option or its value in
        // this context. So we simply use the _ varible, which is
        // a place holder Rust uses to indicate we are intentionally
        // ignoring the expression / result that would normally need
        // to go here
        _ => 20,
    };

    // Here, 'res' will be an integer value of 0 if the value variable
    // is none, and 20 if it is anything else (ie: Some<T>)
    println!("Result is {}", res);
}

fn if_let_with_extract_sample() {
    let money = Coin2::Quarter(USState::Florida);
    let money2 = Coin2::Quarter(USState::Alaska);

    // Is there some way to combine the if-let and match statements here?
    // example below doesn't work:
    // if let Coin2::Quarter(coin_type) = money && coin_type = USState::Florida {
    //     println!("Found Florida coin");
    // }

    if let Coin2::Quarter(coin_type) = money {
        match coin_type {
            USState::Florida => println!("Found Florida coin"),
            _ => println!("Found some other coin"),
        }
    }

    if let Coin2::Quarter(coin_type) = money2 {
        match coin_type {
            USState::Florida => println!("Found Florida coin"),
            _ => println!("Found some other coin"),
        }
    }
}
fn main() {
    //show_value();
    //extract_enum_data();
    //ignore_enum_data();

    //problem_case1_solution();
    //match_with_option_example();
    //catch_all_with_value();
    //catch_all_with_ignore();

    //******************
    problem_case1();
    if_let_with_extract_sample();
}
