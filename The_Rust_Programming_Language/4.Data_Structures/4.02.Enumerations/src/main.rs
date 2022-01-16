#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

fn enums() {
    let c = Color::CmykColor {
        cyan: 2,
        magenta: 0,
        yellow: 222,
        black: 0,
    };
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 0,
        } => println!("Black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        _ => println!("other color"),
    }
}

fn main() {
    enums();
}
