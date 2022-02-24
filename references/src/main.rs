fn main() {
    //problem_case1();
    //solution_case1();
    //mutable_reference();

    //mixed_references1();
    // mixed_references2();
    // mixed_references3();
    // mixed_references4_unknown_cause();
    //mixed_references5_unknown_cause();
    //mixed_references6();

    //lifetime_sample();
    mixed_references7();
}

fn mixed_references1() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // This is fine because we are using immutable references
    println!("Values are {} and {}", r1, r2);

    // This is fine because 's' is a mutable String that is owned by the current scope
    s.push_str(" fubar");

    // but trying to reference an immutable reference to the String after it has been
    // modified causes a compiler error:
    //println!("New value is {}", r2);
}

fn mixed_references2() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // This is fine because we are using immutable references
    println!("Values are {} and {}", r1, r2);

    // taking 1 mutable reference to a string is OK
    let r3 = &mut s;

    // taking a second immutable reference to the same string is not OK
    //let r4 = &mut s;
}

fn mixed_references3() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // This is fine because we are using immutable references
    println!("Values are {} and {}", r1, r2);

    // taking 1 mutable reference to a string is OK
    let r3 = &mut s;

    // trying to modify the original String while there is a valid, mutable reference
    // still in scope is invalid and triggers a compiler error
    //s.push_str(" more suffix");

    // but while the mutable reference is in scope, we can safely modify it here
    r3.push_str(" more suffix");
}

fn mixed_references4_unknown_cause() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r3 = &mut s;

    // this is fine because we are using the mutable reference only
    println!("{}", r3);
    // TBD:
    // but this is problematic for some reason
    // also, uncommenting this line causes rust-analyser to flag the
    // line declaring r3 above, which is weird because that line
    // has no problem on it
    //
    // I think the idea here is that using r3 (the mutable reference) here
    // we could potentially be modifying the value while r1 is still in scope
    // essentially invalidating the data r1 points to. However, in this case this
    // isn't an issue because println doesn't modify r3, so it is actually thread
    // safe. So maybe this is a bug in the NLL (Non Lexical Lifetime) logic.

    //println!("{}", r1);
}

fn mixed_references5_unknown_cause() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    //let r4 = &mut s;
    {
        let r3 = &mut s;
        // This is problematic because we just took a mutable reference
        //println!("Immutables are {} and {}", r1, r2);
        //r3.push_str(" new stuff");
        println!("{}", r3);
    }
    // using the immutable references outside of the scope of the mutable reference
    // here should be fine, but for some reason it isn't
    // TBD
    // I think it has something to do with "overlapping scopes" (ie: the immutable refs
    // were created before the new scope block, and then used afterwards, so the two
    // scopes overlap one another). I'm just not sure how/why this is a problem.
    //
    // Thought: NLL (Non Lexical Lifetimes) look to be magic done by the compiler to
    // try and keep your code thread safe, but for people not writing multi threaded code
    // it creates a layer of complexity that you MUST understand in order to use the
    // language, which is unfortunate
    //
    //println!("Immutables are {} and {}", r1, r2);
    // more interestingly, see what DOES work in mixed_references6 below
}

fn mixed_references6() {
    let mut s = String::from("hello");

    let r3 = &mut s;
    r3.push_str(" even More stuff");

    {
        let r1 = &mut s;
        // This is problematic because we just took a mutable reference
        //println!("Immutables are {} and {}", r1, r2);
        r1.push_str(" new stuff");
        println!("{}", r1);
    }
    let r2 = &mut s;
    r2.push_str(" More stuff");
    println!("New value is {}", r2);
}

fn mixed_references7() {
    let mut s = String::from("Hello");

    // Putting the declaration for the immutable reference before
    // our scope block causes problems. Something to do with overlapping
    // scopes. In the scope block below we have a mutable reference to 's'
    // which conflicts with having an immutable reference in the same scope
    // let r1 = &s;
    {
        let r2 = &mut s;
        println!("Value of mutable is {}", r2);
    }

    // However, if we define our immutable reference here, after our scope block,
    // the it is OK because the mutable reference r2 above is no longer in scope
    // and has been dropped by this point
    let r1 = &s;
    println!("Value of immutable is {}", r1);
}

fn mutable_reference() {
    let mut s = String::from("Dear John");

    change_str(&mut s);
    show_string(&s);
    println!("The new value of s is {}", s);
}
fn show_string(data: &String) {
    println!("The value of the data parameter is {}", data);
}
fn change_str(data: &mut String) {
    data.push_str(", I love you!");
}
fn solution_case1() {
    let s1 = String::from("Johnny Test");

    // Here we pass a reference to our string to the 'calculate_length2' method
    // so that ownership remains here in the calling code
    let len = calculate_length2(&s1);

    // And now we can still use 's1' because we've retained ownership here
    println!("The length of {} is {}", s1, len);
}

fn calculate_length2(data: &String) -> usize {
    data.len()
}

fn problem_case1() {
    let s1 = String::from("First string");

    // Here we pass ownership of s1 to 'calculate_length', so instead
    // of just returning the length from the function we need to also
    // return ownershup of the passed in String back to the caller so
    // it can use it
    let (s2, len) = calculate_length(s1);

    // Example: the following line won't work because 's1' passed ownership
    // to 'calculate_length' and would have been deleted when it went out of
    // scope (except that it has been transferred back into s2)
    //println!("The length of {} is {}", s1, len);

    println!("The length of {} is {}", s2, len);
}
fn calculate_length(data: String) -> (String, usize) {
    let len = data.len();
    (data, len)
}

/*fn lifetime_sample() {
    let s = dangle();
}

fn dangle() -> &String {
    let local = String::from("Hello world");

    // Here we are returning a reference to a symbol that is scoped to this
    // function. So when the function completes, it will see the variable is
    // still owned by the function and essentially clean up the memory.
    // However, this would invalidate the reference being passed back to the
    // caller. So we get a compile time error.
    &local
}*/
