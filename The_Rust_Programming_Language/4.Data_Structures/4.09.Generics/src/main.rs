#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

#[allow(dead_code)]
struct Line<T, U> {
    start: Point<T, U>,
    end: Point<T, U>,
}

#[allow(unused_variables)]
fn generics() {
    let a = Point { x: 0, y: 4 };
    let b = Point { x: 1.2, y: 3.4 };

    let c: Point<u16, u32> = Point { x: 0, y: 4 };

    let line = Line {
        start: Point { x: 0, y: 0.5 },
        end: Point { x: 1, y: 0.08 },
    };
}

fn main() {
    generics();
}
