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
fn main() {
    let home = ip_with_varied_data::v4(127, 0, 0, 1);
    let loopback = ip_with_varied_data::v6(String::from("::0"));

    show_ip(&home);
    show_ip(&loopback);

    home.show();
    loopback.show();
}
