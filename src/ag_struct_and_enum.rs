#![allow(dead_code)]

// basic struct
#[derive(Debug)]    // this tells rust how to print the struct
struct User {
    _username: String,
    _email: String,
    _active: bool,
    _sign_in_count: u64,
}

// functions defined for a struct are called the methods of the struct. This can be done using the impl keyword.
impl User {
    fn get_username(&self) -> &str  {
        &self._username[..]  // we slice the string so it is copied (I think)
    }
}

// basic enum
enum IpAddrKind {
    V4,
    V6,
}

// enums can take value, for example
enum IpAddr {
    V4(String),
    V6(String),
}

// enums can mix types as well
enum Message {
    Quit,
    Move { _x: i32, _y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// you can implement methods for Enums as well.
impl IpAddr {
    fn print_summary(&self) {
        match self {
            Self::V4(_addr) => {
                if let IpAddr::V4(_addr) = IpAddr::V4(String::from("127.0.0.1")) {
                    println!("IpV4 Home address");
                } else {
                    println!("IpV4 address: {}", _addr);
                }
            },
            Self::V6(_addr) => {
                println!("IpV6 address: {}", _addr);
            },
        }
    }
}


fn main(){
    let user1 = User {
        _email: String::from("haha@hehe.net"),
        _username: String::from("AAAAHHHHHH"),
        _active: true,
        _sign_in_count: 1,
    };

    println!("{}", User::get_username(&user1));
    println!("{:?}", user1);    // more compact output
    println!("{:#?}", user1);   // better for more complex structs

    let a=2;
    let b=3;

    let _m1 = Message::Quit;
    let _m2 = Message::Move{_x:a, _y:b};
    let _m3 = Message::Write(String::from("hello"));
    let _m4 = Message::ChangeColor(255, 255, 0);

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    IpAddr::print_summary(&home);
    IpAddr::print_summary(&loopback);
}