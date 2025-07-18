fn main() {
    let rectangle1: Rectangle = Rectangle{
        width: 30,
        height: 40,
    };

    let rectangle2: Rectangle = Rectangle{
        width: 20,
        height: 30,
    };

    let rectangle3: Rectangle = Rectangle{
        width: 10,
        height: 40,
    };

    println!("Area of rectangle1 is:- {}", rectangle1.area());
    println!("Area of rectangle2 is:- {}", rectangle2.area());
    println!("Area of rectangle3 is:- {}", rectangle3.area());

    println!("Can rect1 hold rect2:- {}", rectangle1.can_hold(&rectangle2));
    println!("Can rect2 hold rect3:- {}", rectangle2.can_hold(&rectangle3));
    println!("Can rect1 hold rect3:- {}", rectangle1.can_hold(&rectangle3));

    let sq: Rectangle = Rectangle::square(32);
    println!("{:#?}", sq);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}