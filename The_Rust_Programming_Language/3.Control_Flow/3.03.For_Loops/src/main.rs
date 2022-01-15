fn for_loop() {
    // for x in 1..11 {
    //     println!("x = {}", x);
    // }
    println!("First LOOP");
    for x in 1..=10 {
        if x == 3 {
            continue;
        }
        if x == 8 {
            break;
        }
        println!("x = {}", x);
    }
    println!("Second LOOP");
    for (pos, y) in (30..41).enumerate() {
        println!("pos = {}, y = {}", pos, y);
    }
}

fn main() {
    for_loop();
}
