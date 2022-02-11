#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    println!("-----Rust Conditional Statements-----");

    let some_bool = true;
    let some_int = 30;

    if some_bool == true {
        println!("Hit If branch");
    } else if some_bool == false && some_int == 30 {
        println!("Hit If Else branch");
    } else {
        println!("Hit else branch");
    }

    let var_from_inline = if some_int == 9 {
        300
    } else if some_int == 30 {
        println!("This is if else branch");
        30
    } else {
        400
    };

    // Match Statements
    match some_bool {
        true => {
            println!("Hit true branch");
        }
        false => {
            println!("Hit false branch");
        }
    }
    match some_int {
        0 => println!("Hit 0 branch"),
        1 | 2 => println!("1 or 2"),
        tmp @ 5..=30 => println!("value of tmp is {}", tmp),
        31..=100 => println!("Between 31 and 100 branch"),
        _ => println!("Else branch"),
    }
}
