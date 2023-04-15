fn main() {
    println!("Hello bro, {}", 8);
    println!("Number {}", number());

    println!("Add {}", add(1, 2));
    let my_number;
    {
        my_number = 32;
        println!("My Number 1: {}", my_number);
    }
    {
        let my_number = 32;
        println!("My Number: {}", my_number);
    }
    println!("My Number 2: {}", my_number);
    let new_number = {
        let a = 1+3;
        let b = a*a + a*a + 2 * a * a; 
        a + b
    };
    println!("New Number: {}", new_number);
    let tuple = {
        8;
    };
    println!("Tuple: {:?}", tuple);
}

fn number() -> u8 {
    8
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}