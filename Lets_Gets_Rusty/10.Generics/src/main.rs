/*
============5==============
 */
#[allow(dead_code)]
enum Option<T> {
    Some(T),
    None,
}

#[allow(unused_variables)]
fn main() {
    let integer = Option::Some(5);
    let float = Option::Some(5.0);
}

/*
============4==============
 */
/* #[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
 */
/*
============4==============
 */
/* #[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}
impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}
impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

#[allow(unused_variables)]
fn main() {
    let p1 = Point { x: 5, y: 10 };
    p1.x();
    let p2 = Point { x: 5.1, y: 10.2 };
    p2.y();
    p2.x();
} */

/*
============3==============
 */
/* #[allow(unused_variables)]
fn main() {
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
 */

/*
============2==============
 */
/* #[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

#[allow(unused_variables)]
fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let p3 = Point { x: 5, y: 10.0 };
}
 */

/*
============1==============
 */
/* fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);
    let char_list = vec!['y', 'u', 'a', 'q'];
    let largest = get_largest(char_list);
    println!("The largest number is {}", largest);
}
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for element in list {
        if element > largest {
            largest = element;
        }
    }
    largest
}
 */
