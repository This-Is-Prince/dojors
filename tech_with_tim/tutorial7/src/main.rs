fn main() {
    println!("Tutorial 7 - Conditions and Control Flow (if_else if_else).");

    let cond = 2 <= 2;
    println!("{}", cond);

    // let cond=2<2.2; // ERROR, expected integer, found floating-point number
    println!("{}", cond);

    let cond = (2 as f32) > 2.2;
    println!("{}", cond);

    let cond = true && cond;
    println!("{}", cond);

    let cond = true || cond;
    println!("{}", cond);

    let food = "cookie";

    if food == "cookie" {
        println!("This is true")
    }
}
