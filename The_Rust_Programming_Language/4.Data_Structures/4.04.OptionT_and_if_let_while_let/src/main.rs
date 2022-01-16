fn main() {
    let x = 3.0;
    let y = 2.0;
    // Option -> Some(v) | None
    let result = if y != 0.0 { Some(x / y) } else { None };
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("can't divide by zero"),
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }
    let mut x = 0;
    while let Some(z) = result {
        println!("z = {}, x = {}", z, x);
        x += 1;
        if x == 10 {
            break;
        }
    }
}
