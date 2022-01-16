fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);

    println!("a = {:?}", a);

    // usize isize
    // let idx: usize = a.len() - 1;
    let idx: usize = 0;

    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    // Option
    match a.get(3) {
        Some(x) => println!("a[3] = {}", x),
        None => println!("error, no such element"),
    }

    for x in &a {
        println!("{}", x);
    }

    a.push(77);
    println!("{:?}", a);

    // Some(77)
    let last_elm = a.pop(); // Option
    println!("last elem is {:?},  a = {:?}", last_elm, a);
    // match last_elm {
    //     Some(x) => println!("{}", x),
    //     None => println!("no element"),
    // }

    while let Some(x) = a.pop() {
        println!("x = {}", x);
    }
}

fn main() {
    vectors();
}
