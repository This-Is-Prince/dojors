#[allow(unused_variables)]

fn main() {
    let some_bool = true;
    let some_int = 30;
    let some_int2 = 50;

    // if some_bool == true {
    //     println!("Hit If branch");
    // }
    // if some_bool {
    //     println!("Hit If branch");
    // } else {
    //     println!("Hit Else branch");
    // }

    if some_bool || (some_int > 100 && some_int2 == 200) {
        println!("Hit If branch");
    } else {
        println!("Hit Else branch");
    }

    let var_from_inline = if some_int == 9 { 300 } else { 400 };
    println!("var_from_inline is {}", var_from_inline);

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
        1..=100 => println!("Between 1 and 100"),
        _ => println!("Else branch"),
    }

    match some_int {
        0 => {
            println!("Hit 0 branch")
        }
        1..=100 => {
            println!("Between 1 and 100")
        }
        _ => {
            println!("Else branch")
        }
    }

    let var_from_match = match some_bool {
        true => 10,
        false => 20,
    };
    println!("{}", var_from_match);

    let var_from_match2 = match some_int {
        0 => 0,
        1 | 2 => 100,
        _ => 200,
    };
}
