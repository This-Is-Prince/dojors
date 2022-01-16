fn use_slice(slice: &mut [i32]) {
    println!("first elm = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5];

    println!("{:?}", data);
    use_slice(&mut data[1..4]);
    // use_slice(&mut data);
    println!("{:?}", data);
}

fn main() {
    slices();
}
