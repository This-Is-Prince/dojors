fn main() {
    let x = 3.0;
    let y = 2.0;

    // Option -> Some(z) / None
    let result = if y != 0.0 { Some(x / y) } else { None };

    if let Some(z) = result {
        println!("value of z is {}", z);
    }

    let mut x = 0;
    while let Some(z) = result {
        println!("x = {}, z = {}", x, z);
        x += 1;
        if x > 10 {
            break;
        }
    }
}
