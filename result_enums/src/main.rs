use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn simple_match() {
    // Here the result, f, is a Result<T, E> enumeration
    let f = File::open("hello.txt");

    // here we unwrap the enumeration using a match statement, and assign
    // the _f variable the file handle returned on success
    let _f = match f {
        // return / unwrap the file handle on success
        Ok(file) => file,
        // panic / crash the app on failure, printing the error to the console
        Err(err) => panic!("Some unknown error occurred {:?}", err),
    };
}

fn nested_match() {
    let f = File::open("hello.txt");

    let _f = match f {
        // on success, unwrap the file and return it to _f
        Ok(file) => {
            println!("Opened existing file");
            file
        }
        // on error, unwrap the error to look more closely...
        Err(err) => match err.kind() {
            // if the error is "not found" we create a new file...
            ErrorKind::NotFound => match File::create("hello.txt") {
                // the creation method may fail, so we unwrap the file handle
                // and return it back up to _f
                Ok(file) => {
                    println!("Created file");
                    file
                }
                // on create error give up and crash the app
                Err(err) => panic!("Failed to locate OR create file {:?}", err),
            },
            other_error => {
                // for any other type of error opening the file, other than
                // not found (ie: access denied, etc.) fail outright
                panic!("Some unknown error occurred {:?}", other_error)
            }
        },
    };

    // By this point we know the program has either exited due to a panic, or the _f
    // variable contains a valid value, and the compiler will validate this for you based
    // on the exhaustive requirement of match statements and enumerations enforced by Rust
}

fn load_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    let f = File::open("hello.txt");

    // f-overshadowed here has to be mutable because the
    // read_to_string method requires it
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    // the read_to_string operation may fail, so we need to look at
    // the Result returned
    match f.read_to_string(&mut s) {
        // On success, read_to_string returns the number of bytes read
        // as a numeric value (usize)
        Ok(size) => {
            println!("Successfully read {} bytes", size);
            Ok(s)
        }
        Err(e) => Err(e),
    } // NOTE: here we are using the match expression to return the value back to the
      // caller. In this case we return a Result enum, so either Ok() variant containing
      // the string data read, or an Err() variant containing the io::Error result, propagating
      // the status information back to the caller
}
fn load_username_from_file2() -> Result<String, io::Error> {
    // This method does the same thing as the previous one with match
    // expressions replaced with the ? operator. It does nearly the same
    // thing apparently, but with some additional logic for converting
    // error types or something...
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn load_username_from_file3() -> Result<String, io::Error> {
    // Shortens the code further by combining statements on a single line
    // still does the same thing
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
fn propagation_sample() {
    //let t = load_username_from_file();
    let t = load_username_from_file3();
    let mut s = String::new();
    fs::read_to_string(&mut s);
    match t {
        Ok(name) => println!("User name loaded: {}", name),
        Err(e) => println!("Failed to load name {}", e),
    };
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(new_value: i32) -> Guess {
        if new_value < 1 || new_value > 100 {
            panic!("Value out of range 1..100: {}", new_value);
        }
        Guess { value: new_value }
    }

    pub fn value(&self) -> i32 {
        println!("value in getter is {}", self.value);
        self.value
    }
}

fn guess_example() {
    let mut a = Guess::new(4);
    println!("First vlue is {}", a.value);
    // This next line should not be possibe because `value` is a private property
    a.value = 10;
    println!("Value is now {}", a.value);
    println!("This is different {}", a.value());
    //let b = Guess::new(999);
    //println!("Should not see me {}", b.value);
}

// Box<dyn Error> basically means any type that represents any type of error
// Result<(), Box..> basically allows an empty type (ie: success) or an
// error type back the caller. When main() returns an empty tuple Rust sets
// the return code to 0. If a box<Error> type is returned it will set the return
// value to some non-0 value
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    propagation_sample();
    guess_example();
    Ok(())
}
