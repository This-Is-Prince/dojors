fn main() {
    let width1 = 30;
    let height1: i32 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    
    let rect1: Rectangle = Rectangle { width: dbg!(30 * 1), height: 50 };
    println!("The area of the rectangle is {} square pixels.", area1(&rect1));
    println!("rect1 is {rect1:#?}");
    dbg!(&rect1);
}

fn area(width1: i32, height1: i32) -> i32 {
    width1 * height1
}

fn area1(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}