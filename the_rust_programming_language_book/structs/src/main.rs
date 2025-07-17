
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1: User = User{
        active: false,
        username: String::from("Prince"),
        email: String::from("prince@in"),
        sign_in_count: 2,
    };

    println!("{:?}", user1.active);
    println!("{:?}", user1.username);
    println!("{:?}", user1.email);
    println!("{:?}", user1.sign_in_count);

    let black = Color(0, 0, 0);
    let point: Point = Point(0, 0, 0);

    let Point(x, y, z) = point;
    println!("{x} {y} {z}");

    let Color(r, g, b) = black;
    println!("{r} {g} {b}");

    let _subject = AlwaysEqual;
    // println!("{:?}", subject);
}
