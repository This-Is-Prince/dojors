fn main() {
    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;

    route(four);
    route(six);

    let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));

    println!("{:#?}\n{:#?}", home, loopback);

    let mut opt: Option<u8> = Some(5);

    if let Some(num) = opt {
        println!("Value is {num}");
    } else {
        println!("Value is None");
    }

    opt = None;

    match opt {
        Some(value) => {
            println!("Value is {value}");
            opt = None;
        },
        None => {
            println!("Value is None");
            opt = Some(5);
        },
    }

    match opt {
        Some(value) => {
            opt = Some(value + 1);
            println!("Value is {value}");
        },
        _ => (),
    }

    let new_opt: i32 = if let Some(v) = opt {
        v as i32
    } else {
        0
    };

    println!("{:#?}", new_opt);
}

fn hi() -> Option<String> {
    let var = Some(8);

    let Some(v) = var else {
        return None;
    };

    Some(String::from("Mil Gya"))
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("{:#?}", ip_kind);
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}