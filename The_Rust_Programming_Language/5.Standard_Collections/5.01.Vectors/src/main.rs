fn vectors() {
    // !!!!!REMINDER VARIABLES ARE ALWAYS IMMUTABLE IN RUST
    // let a = Vec::new();
    // a.push(1); // Error

    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    // println!("a = {:#?}", a);
    println!("a = {:?}", a);
    a.push(44);
    // println!("a = {:#?}", a);
    println!("a = {:?}", a);

    println!("a[0] = {}", a[0]);

    // usize, isize
    // let idx: i32 = 0;
    // println!("a[0] = {}", a[idx]); //Error, index must be of type usize in rust
    let idx: usize = 0;
    println!("a[0] = {}", a[idx]);
    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    // let idx: usize = 111;
    // println!("a[0] = {}", a[idx]); // panic at compile time

    // Another safer method
    // get method returns Option type two possible value Some(value) or None
    let mut idx = 6;
    match a.get(idx) {
        Some(x) => println!("a[{1}] = {}", x, idx),
        None => println!("error, no such element"),
    }
    idx = 3;
    match a.get(idx) {
        Some(x) => println!("a[{1}] = {}", x, idx),
        None => println!("error, no such element"),
    }

    for x in &a {
        println!("{}", x);
    }
    a.push(44);
    println!("{:?}", a);

    // Removing Elements
    let last_elem = &mut a.pop(); // pop returns Option
    match last_elem {
        Some(v) => {
            println!("last element is {}", v);
            *v = 111;
            println!("last element is {}", v);
        }
        None => {
            println!("there is no last element")
        }
    }
    println!("{:?}", a);
}

fn main() {
    vectors();
}
