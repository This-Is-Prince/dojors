#[allow(unused_mut)]
fn main() {
    let print_vector = |x: &Vec<i32>| {
        println!("x[0] = {}", x[0]);
    };

    let v = vec![3, 2, 1];
    print_vector(&v); // v borrowed by print_vector function
    println!("{:?}", v);

    let mut a = 40;
    let c = &a;
    println!("c = {}", c);
    {
        let b = &mut a;
        *b += 2;
    }
    println!("a = {}", a);
    let mut z = vec![3, 2, 1];
    for i in &z {
        println!("i = {}", i);
        // z.push(5); // ERROR, cannot borrow `z` as mutable because it is also borrowed as immutable
    }
}
