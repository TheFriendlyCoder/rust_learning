fn basic_conditional() {
    let number = 9;

    // "if expressions" suggests that if statements return values - do they?
    //      they don't end with semicolons, so maybe they do?
    //      yes they do - see if_expression()
    if number > 5 {
        // branch blocks / conditions are typically referred to as "arms"
        // comes from terminology related to "match" expressions
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

fn if_expression() {
    let condition = true;
    // since 'if' keywards are expressions, you can use them on the
    // right side of assignment statements
    // Notice there are no semicolons inside the arms of the if expression
    let x = if condition { 4 } else { 17 };
    println!("Value of X is {}", x);

    // the data types in each arm of an if expression must be the same
    // the following statement produces an error
    //let y = if condition { 4 } else { "yes" };
}
fn main() {
    basic_conditional();
    if_expression();
}
