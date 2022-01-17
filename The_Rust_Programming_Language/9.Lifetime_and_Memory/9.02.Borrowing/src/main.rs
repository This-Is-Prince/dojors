fn main() {
    let print_vector = |x: &Vec<i32>| {
        println!("x[0] = {}", x[0]);
    };

    let v = vec![3, 2, 1];
    print_vector(&v); // v borrowed by print_vector function
    println!("{:?}", v);

    let mut a = 40;
    let b = &mut a;

    *b += 2;
    println!("a = {}", a);
}
