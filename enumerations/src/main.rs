enum simple_ip {
    v4,
    v6,
}

enum ip_with_data {
    v4(String),
    v6(String),
}

// NOTE: to make an enum printable it has to define a Display
// or Debug interface (trait??)
#[derive(Debug)]
enum ip_with_varied_data {
    v4(u8, u8, u8, u8),
    v6(String),
}

impl ip_with_varied_data {
    fn show(&self) {
        println!("Showing what IP is {:?}", self);
    }
}

fn show_ip(address: &ip_with_varied_data) {
    // Here we can pass an instance of a v4 or v6 IP address
    // and the print statement will just work

    println!("IP Address value is {:?}", address);
}

fn ip_address_sample() {
    let home = ip_with_varied_data::v4(127, 0, 0, 1);
    let loopback = ip_with_varied_data::v6(String::from("::0"));

    show_ip(&home);
    show_ip(&loopback);

    home.show();
    loopback.show();
}

fn get_optional() -> Option<u8> {
    //Some(42)
    None
}

fn option_enum_sample() {
    println!("{:?}", get_optional());
    if get_optional().is_none() {
        // expect() will check to see if the option has a non-empty value.
        // if the value is empty it will "panic" and mention the message
        // provided (ie: "fubar2")
        println!("Got a none value: {}", get_optional().expect("fubar2"));
    }
    if get_optional().is_some() {
        // if the option has a valid value, expect() returns a reference
        // to the value included inside the option so it can be used
        // later. Here we get a valid u8 integer value to pass to the print
        // macro
        println!("Got a valid value of {}", get_optional().expect("fubar"));
    }
}
fn main() {
    ip_address_sample();
    option_enum_sample();
}
