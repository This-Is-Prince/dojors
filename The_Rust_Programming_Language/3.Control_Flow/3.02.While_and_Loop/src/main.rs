fn while_and_loop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        } else if x == 512 {
            break;
        }

        println!("x = {}", x);
    }

    let mut y = 1;
    loop
    // while true
    {
        y *= 2;
        println!("y = {}", y);

        if y >= 10000 {
            break;
        }
    }
}

fn main() {
    while_and_loop();
}
