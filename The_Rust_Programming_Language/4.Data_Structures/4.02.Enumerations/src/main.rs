#[allow(dead_code)]

enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, f32),
    HSL {
        hue: u16,
        saturation: u8,
        lightness: u8,
    },
}

fn enums() {
    let color1 = Color::RGB(0, 25, 0);
    let color2 = Color::RGBA(0, 25, 220, 0.5);
    let color3 = Color::HSL {
        hue: 0,
        saturation: 100,
        lightness: 50,
    };
    let color4 = Color::HSL {
        hue: 120,
        saturation: 100,
        lightness: 50,
    };
    match color4 {
        Color::Red => {
            println!("color is Red");
        }
        Color::Green => {
            println!("color is Green");
        }
        Color::Blue => {
            println!("color is Blue");
        }
        Color::RGB(r, g, b) => {
            println!("color is rgb({}, {}, {})", r, g, b);
        }
        Color::RGBA(r, g, b, a) => {
            println!("color is rgba({}, {}, {}, {})", r, g, b, a);
        }
        Color::HSL {
            hue: 0,
            saturation: 100,
            lightness: 50,
        } => {
            println!("color is hsl(0, 100, 50) which means red");
        }
        Color::HSL {
            hue,
            saturation,
            lightness,
        } => {
            println!("color is hsl({}, {}, {})", hue, saturation, lightness);
        }
    }
    match color3 {
        Color::Red => {
            println!("color is Red");
        }
        Color::Green => {
            println!("color is Green");
        }
        Color::Blue => {
            println!("color is Blue");
        }
        Color::RGB(r, g, b) => {
            println!("color is rgb({}, {}, {})", r, g, b);
        }
        Color::RGBA(r, g, b, a) => {
            println!("color is rgba({}, {}, {}, {})", r, g, b, a);
        }
        Color::HSL {
            hue: 0,
            saturation: 100,
            lightness: 50,
        } => {
            println!("color is hsl(0, 100, 50) which means red");
        }
        Color::HSL {
            hue,
            saturation,
            lightness,
        } => {
            println!("color is hsl({}, {}, {})", hue, saturation, lightness);
        }
    }
    match color1 {
        Color::Red => {
            println!("color is Red");
        }
        Color::Green => {
            println!("color is Green");
        }
        Color::Blue => {
            println!("color is Blue");
        }
        Color::RGB(r, g, b) => {
            println!("color is rgb({}, {}, {})", r, g, b);
        }
        Color::RGBA(r, g, b, a) => {
            println!("color is rgba({}, {}, {}, {})", r, g, b, a);
        }
        Color::HSL {
            hue: 0,
            saturation: 100,
            lightness: 50,
        } => {
            println!("color is hsl(0, 100, 50) which means red");
        }
        Color::HSL {
            hue,
            saturation,
            lightness,
        } => {
            println!("color is hsl({}, {}, {})", hue, saturation, lightness);
        }
    }
    match color2 {
        Color::Red => {
            println!("color is Red");
        }
        Color::Green => {
            println!("color is Green");
        }
        Color::Blue => {
            println!("color is Blue");
        }
        Color::RGB(r, g, b) => {
            println!("color is rgb({}, {}, {})", r, g, b);
        }
        Color::RGBA(r, g, b, a) => {
            println!("color is rgba({}, {}, {}, {})", r, g, b, a);
        }
        Color::HSL {
            hue,
            saturation,
            lightness,
        } => {
            println!("color is hsl({}, {}, {})", hue, saturation, lightness);
        }
    }
}

fn main() {
    enums();
}
