#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);
    let p2 = Point { x: 5.0, y: 10.0 };

    let my_line = Line { start: p, end: p2 };
    println!("myLine is {:?}", my_line);
    println!("start is {:?}", my_line.start);
    println!("end is {:?}", my_line.end);
}

fn main() {
    structures();
}
