fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_params(2,5);

    let x= five();
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("This is another function.");
}

fn another_function_with_params(x:i32, y:i32) {
    println!("The sum of {} and {} is {}", x, y, x + y);
}

fn five() ->i32 {
    5
}