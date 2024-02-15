#![allow(unused)]

struct Color;

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKindString {
    V4(String),
    V6(String),
}

enum IpAddrKindInt {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddrKindStruct {
    V4(Ipv6Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(i32),
}

fn use_enum() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
}

fn use_addr() {
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn use_enum_string() {
    let _home = IpAddrKindString::V4(String::from("localhost"));
    let __home = IpAddrKindInt::V4(127, 0, 0, 1);
    let _loopback = IpAddrKindInt::V6(String::from("::1"));
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        _ => 100,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    x.map(|i| i + 1)
}

fn dice(dice: u8) {
    match dice {
        3 => {}
        7 => {}
        // catch param, or use _
        oth => {
            println!("{}", oth)
        }
    }
    match dice {
        3 => {}
        7 => {}
        _ => {
            println!("re roll")
        }
    }
}

#[allow(unused)]
fn if_constitute() {
    let mut count = 0;
    let coin = Coin::Dime;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("{}", count);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        println!("{}{}", file!(), line!());
    }
}
