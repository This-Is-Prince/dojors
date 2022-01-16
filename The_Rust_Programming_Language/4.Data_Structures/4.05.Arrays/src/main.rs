fn print_array_with_loop(arr: &[i32]) {
    println!("print array with loop");
    let mut index = 0;
    loop {
        println!("a[{}] = {}", index, arr[index]);
        index += 1;
        if index >= arr.len() {
            break;
        }
    }
}
fn print_array_with_while(arr: &[i32]) {
    println!("print array with while");
    let mut index = 0;
    while index < arr.len() {
        println!("a[{}] = {}", index, arr[index]);
        index += 1;
    }
}

fn print_array_with_for(arr: &[u16]) {
    for i in 0..arr.len() {
        println!("arr[{}] = {}", i, arr[i]);
    }
}

fn array() {
    // let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    let mut a = [1, 2, 3, 4, 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a);
    print_array_with_loop(&a);
    print_array_with_while(&a);

    if a != [321, 2, 3, 4, 5] {
        println!("does not match");
    } else {
        println!("array match")
    }

    let b = [1u16; 10];
    print_array_with_for(&b);

    println!("b took up {} bytes", std::mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];

    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn main() {
    array();
}
