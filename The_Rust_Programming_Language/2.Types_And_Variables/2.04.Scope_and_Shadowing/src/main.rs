fn scope_and_shadowing() {
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);
        println!("inside a = {}", a);
        let a = 2;
        println!("inside after shadowing a = {}", a);
    }
    // println!("outside, b = {}", b); // Error
    println!("outside a = {}", a);
}

fn main() {
    scope_and_shadowing();
}
