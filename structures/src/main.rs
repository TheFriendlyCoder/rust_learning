#[derive(Debug)] // <-- allows us to println("{:?}") instances of this struct
struct User {
    name: String,
    active: bool,
    count: usize,
    email: String,
}
fn main() {
    //simple_example();
    //method_sample();
    //shorthand_sample();
    //shared_references();
    method_demo();
}

fn simple_example() {
    let new_user = User {
        active: true,
        count: 0,
        name: String::from("Kevin Phillips"),
        email: String::from("thefriendlycoder@gmail.com"),
    };

    println!("New user name is {}", new_user.name);
    // Can't print a struct unless we define some additional boilerplate
    //println!("User data is {}", new_user);

    // debug formatting also doesn't work unless you add the debug trait
    println!("User data is {:?}", new_user);

    // show debug output in a more user friendly format with {:#?}
    println!("User data is {:#?}", new_user);

    // print debug output to stderr and includes file and line number of
    // the statement
    dbg!(&new_user);

    // dbg returns a reference to the original input object
    let other_user = dbg!(&new_user);
    println!("New user is {:#?}", new_user);
    println!("Other user is {:#?}", other_user);
}

fn method_sample() {
    let new_user = create_user1("Kevin Phillips", "thefriendlycoder@gmail.com");
    println!("New user count {}", new_user.count);
}
fn create_user1(user: &str, email: &str) -> User {
    User {
        name: String::from(user),
        active: true,
        count: 10,
        email: String::from(email),
    }
}

fn shorthand_sample() {
    // Here we name our local variables "name" and "email" to match
    // the names of the struct properties exactly. This allows us to
    // assign the values to the names without having to reference the
    // property name explicitly
    let name = String::from("John Doe");
    let email = String::from("john@fubar.com");

    let new_user = User {
        name, // <-- this is equivalent to "name: name,"
        email,
        count: 20,
        active: false,
    };
    println!("New user email is {}", new_user.email);
}

fn shared_references() {
    let user1 = User {
        name: String::from("Kevin"),
        email: String::from("cciviper@hotmail.com"),
        count: 0,
        active: false,
    };

    let user2 = User {
        email: String::from("thefriendlycoder@gmail.com"),
        ..user1 // inherit all remaining properties from user1 instance
    };

    // NOTE: since the "name" property is of type String, the "..user1" syntax
    // will transfer ownership of the "name" strig on user1 to the new user2
    // variable, essentially invalidating user1
    // thus, the following line will fail to compile:
    //println!("User 1 name is {}", user1.name);

    // but we can use user2 just fine
    println!("User 2 name is {}", user2.name);

    // And we can use user1 properties that were copied by value, like "count"
    println!("User 1 count is {}", user1.count);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Defines an "implementation" or set of methods to attach to the
// Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        // <-- "&self" is shorthand for "self: &self"
        self.width * self.height
    }
}

fn method_demo() {
    let rect = Rectangle {
        width: 20,
        height: 45,
    };

    println!("Area of rect is {}", rect.area());
}
