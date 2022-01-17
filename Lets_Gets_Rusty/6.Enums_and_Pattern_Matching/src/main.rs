#[allow(dead_code)]
#[derive(Debug)]
/*
enum IpAddrKind {
    V4,
    V6,
} */

/* enum IpAddrKind {
    V4(String),
    V6(String),
} */
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(u8, u8, u8, u8),
}

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
impl Message {
    fn some_function() {
        println!("Prince");
    }
}

/* struct QuitMessage; //unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
 */

/* #[allow(dead_code)]
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
} */

#[allow(unused_variables)]
fn main() {
    /*
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    */

    /*
    let localhost = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
    };
    */

    /*
    let localhost = IpAddrKind::V4(String::from("localhost:3000"));
    println!("{:?}", localhost);
     */

    /*
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    println!("{:?}", localhost);
     */

    /*
    let msg = Message::Quit;
    println!("{:?}", msg);
    Message::some_function();
     */

    /*
    ======================
    Option Enum
    ======================
    */

    /*
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let y: Option<i8> = None;
    let sum = x + y.unwrap_or(0);
    println!("sum = {}", sum);
    */

    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));
}

#[allow(dead_code)]
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

/*
#[allow(dead_code)]
fn route(ip_kind: IpAddrKind) {
    println!("ip is {:#?}", ip_kind);
}
 */
