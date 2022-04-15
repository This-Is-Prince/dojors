/* Unit Struct */
#[allow(dead_code)]
struct FileDirectory;

/* Tuple Struct */
#[allow(dead_code)]
#[derive(Debug)]
struct Color(u8, u8, u8);

/* Named Struct */
#[allow(dead_code)]
#[derive(Debug)]
struct SizeAndColor {
    size: u32,
    color: Color, // And we put it in our new named struct
}

#[allow(dead_code)]
#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    println!("Structs");

    let my_color = Color(50, 0, 50); // Make a color out of RGB (red, green, blue)
    println!("The second part of the color is: {}", my_color.1);

    let my_color = Color(50, 0, 50);
    let size_and_color = SizeAndColor {
        size: 150,
        color: my_color,
    };
    println!("{:#?}", size_and_color);

    /* First */
    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");
    let kalmykia = Country {
        population: population,
        capital: capital,
        leader_name: leader_name,
    };
    println!("{:#?}", kalmykia);

    /* Second */
    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");
    let kalmykia = Country {
        population,
        capital,
        leader_name,
    };
    println!("{:#?}", kalmykia);
}
