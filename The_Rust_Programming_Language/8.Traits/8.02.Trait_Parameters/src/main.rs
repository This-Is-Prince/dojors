use std::fmt::Debug;

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

fn print_info(shape: impl Shape) {
    println!("The area is {}", shape.area())
}

fn print_info_with_debug_trait(shape: impl Shape + Debug) {
    println!("{:?}", shape);
    println!("The area is {}", shape.area())
}

fn print_info_with_trait_bound<T: Shape + Debug>(shape: T) {
    println!("{:?}", shape);
    println!("The area is {}", shape.area())
}

fn print_info_with_where_clause<T>(shape: T)
where
    T: Shape + Debug,
{
    println!("{:?}", shape);
    println!("The area is {}", shape.area())
}

fn trait_parameters() {
    let c = Circle { radius: 2.0 };
    print_info(c);

    let c = Circle { radius: 3.0 };
    // print_info_with_debug_trait(c); // ERROR, `Circle` doesn't implement `std::fmt::Debug`
    print_info_with_debug_trait(c);

    let c = Circle { radius: 4.0 };
    print_info_with_trait_bound(c);

    let c = Circle { radius: 5.0 };
    print_info_with_where_clause(c);
}

fn main() {
    println!("---------Trait Parameters---------");

    trait_parameters();
}
