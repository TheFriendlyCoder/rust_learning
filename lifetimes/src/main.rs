#![allow(dead_code)]
#![allow(unused_variables)]

// Here we can't return a reference to a slice because
// the Rust borrow checker can't tell what the lifetime
// is of the return value, but the lifetimes of both
// reference params may be different
//fn longest_broke(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn sample1() {
    // both input params have similar lifetimes
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest1(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn sample2() {
    // here string1 has a longer lifetime than string2
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest1(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn sample3() {
    let string1 = String::from("long string is long");
    let result: &str;
    {
        let string2 = String::from("xyz");
        // This triggers an error because string2 has a smaller scope than string1
        // and so string2 causes an error because the borrow checker knows that
        // variable will be out of scope before it is used
        // result = longest(string1.as_str(), string2.as_str());
    }
    //println!("The longest string is {}", result);
}

// NOTE: Here I've given the y parameter a different, implicit lifetime
// that need not match the lifetime of the return value, so it is
// OK if the 2 input parameters have different lifetimes, so long as
// the returned reference points to the x parameter
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        panic!("Failed")
    }
}

fn sample4() {
    // sample example as sample3 but using longest2() helper method
    // with lifetimes that are compatible with your somple
    let string1 = String::from("long string is long");
    let result: &str;
    {
        let string2 = String::from("xyz");
        result = longest2(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// Here we are getting references to each of the input parameters
// as return values. Each of which may have different lifetimes
fn sizes<'a, 'b>(x: &'a str, y: &'b str) -> (&'a str, &'b str) {
    if x.len() > y.len() {
        (x, y)
    } else {
        //(y, x)
        // TODO: figure out if there is a way to make this work
        panic!("Not supported")
    }
}

fn sample5() {
    let string1 = String::from("long string is long");
    let longstr: &str;
    let shortstr: &str;
    {
        let string2 = String::from("xyz");
        (longstr, shortstr) = sizes(string1.as_str(), string2.as_str());
        // Here we can reference both return values because their lifetimes
        // are both still valid
        println!("The longest string is {}", longstr);
        println!("The shortest string is {}", shortstr);
    }
    // But here we can only use the longstr return value
    // The lifetime of the shortstr return value is inherited from
    // string2 in this case, so it is out of scope when we get here
    // which is why we can't use it
    println!("The longest string is {}", longstr);
    //println!("The shortest string is {}", shortstr);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
    // TBD: is this ever valid syntax - to have a reference member
    //      in a struct?
    //part: &str,
}

fn sample6() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let i: ImportantExcerpt;
    {
        // NOTE: if we move the novel var here inside the code block then
        // we can't reference the string reference outside the code block
        // below. It seems the lifetime of the string slice / reference is
        // the same as the original string itself, so as long as the reference
        // string exists in a larger scope than the reference slice we're good
        //let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("The first sentence is {}", i.part);
    }
    // It's surprising that we can still use the string reference for
    // first_sentence here outside the code block even though it is out
    // of scope
    println!("The first sentence is {}", i.part);
}

// Sample lifetimes on struct implementation
// annotations only are needed on the impl definition
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// Example of lifetie elision rules. The return value from
// this method will have the same lifetime as the 'self' parameter
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> ImportantExcerpt<'a> {
    // Here I had to assign an explicit lifetime to the "announcement"
    // parameter because the elision rules can't deduce what the lifetime
    // should be.
    // Based on my understanding, my current implementation should be
    // giving the return value the same lifetime as "self"
    fn announce_and_return_part2(&self, announcement: &'a str) -> &str {
        println!("Attention please: {}", announcement);
        announcement
    }
}

fn sample7() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let temp2 = String::from("Hello to the world!!!");
    let i: ImportantExcerpt;
    let temp: &str;
    {
        // If I move the declaration for i into the inner block we get a lifetime
        // error because now the 'temp' variable lives longer than i, and below we
        // are assigning 'temp' a borrowed reference to the announcement string
        //let i: ImportantExcerpt;

        // If I define my announcement variable as a static string within the
        // code block I get no lifetime errors. This is because static strings
        // have a static lifetime by default, meaning they exist for the entire
        // duration of the application. So the scope extends beyond this code
        // block
        //let local = "Hello to the world!!!!";

        // Conversely, if I use a heap-allocated string and take a slice reference,
        // the reference will have the same lifetime and the original String
        // This means that that 'temp2' has to have a liftetime that lasts at
        // least as long as the lifetime of the 'temp' variable, because it is
        // used outside the scope of this code block. For example, if I move
        // the declaration of 'temp2' in this code block I'll get a lifetime
        // error:
        //let temp2 = String::from("Hello to the world!!!");
        let local = temp2.as_str();
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("The first sentence is {}", i.part);
        temp = i.announce_and_return_part2(local);
    }

    //println!("The first sentence is {}", i.part);
    // I'm actually surprised that this next line works. From what I understand, we
    // declare a local reference called "local" in the code block above, and this slice
    // will go out of scope after it exits the block. But later we pass this variable
    // to the i.announce_and_return_part2 method, which in turn returns a borrowed reference
    // to "local" and assigns it to the "temp" variable. So by the time we reach here
    // I would have expected the reference slice assigned to "temp" to be invalid since
    // the original source, "local", is out of scope.
    println!("The temporary variable is {}", temp);
}
fn main() {
    // Good example for discussion
    //sample5();

    // Good example for discussion
    //sample6();

    sample7();

    // {
    //     let r;

    //     {
    //         let x = 5;
    //         r = &x;
    //     }

    //     println!("r: {}", r);
    // }
}
