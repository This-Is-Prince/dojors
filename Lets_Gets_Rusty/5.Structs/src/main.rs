/*
#[allow(dead_code)]
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
} */

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[allow(unused_variables)]
fn main() {
    /*
    let mut user1 = User {
        name: "Prince".to_string(),
        email: "prince@abc.com".to_string(),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.name; // Copy
    user1.name = String::from("prince kumar");
    println!("{}", name);
    println!("{:?}", user1);

    let user2 = build_user(String::from("abc@gmail"), String::from("Hello"));

    let user3 = User {
        email: String::from("abcdef@gmail"),
        name: String::from("Hello World"),
        ..user1
    }; */

    /*
    ============
    Tuple Structs
    ============
    */
    /*
    #[derive(Debug)]
     struct Color(i32, i32, i32);
     #[derive(Debug)]
     struct Point(i32, i32, i32);
     let c = Color(11, 22, 33);
     let p = Point(1, 2, 3);
     println!("{:?}", c.0);
     println!("{:?}", c.1);
     println!("{:?}", c.2);
     */

    /*
    let rect = (30, 50);
    println!("The are of the rectangle is {} square pixels.", area(rect));
    */
    /*
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect: {:?}", rect);
    println!("rect: {:#?}", rect);
    println!("The are of the rectangle is {} square pixels.", area(&rect));
    */

    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    println!(
        "The are of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("rect1 can hold rec2: {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rec1: {}", rect2.can_hold(&rect1));

    let rect3 = Rectangle::square(10);
    println!("rec3 is: {:#?}", rect3);
}
/*
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
 */
/*
fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}
 */
/* fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: false,
        sign_in_count: 1,
    }
}
 */
