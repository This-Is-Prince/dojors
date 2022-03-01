#[allow(unused_variables)]
fn main() {
    println!("------------Conditional Statements------------");
    let some_bool = true;

    // if some_bool == true {
    //     println!("Hit If branch");
    // }

    // if !(some_bool == true) {
    //     println!("Hit If branch");
    // }

    // if !some_bool {
    //     println!("Hit If branch");
    // } else {
    //     println!("Hit Else branch");
    // }

    let some_int1 = 30;
    let some_int2 = 50;
    if !some_bool || some_int1 > 100 && some_int2 == 200 {
        println!("Hit If branch")
    } else if some_int1 == 30 {
        println!("Hit If Else branch")
    } else {
        println!("Hit Else branch")
    }

    let var_from_inline = if some_int1 == 9 {
        300
    } else if some_int2 == 50 {
        println!("Test");
        0
    } else {
        400
    };
    println!("var_from_inline = {}", var_from_inline);

    match some_bool {
        true => {
            println!("Hit true branch")
        }
        false => {
            println!("Hit false branch")
        }
    }

    let some_int = 200;
    match some_int {
        0 => println!("Hit 0 branch"),
        1..=100 => println!("Between 1 and 100 branch"),
        var @ _ => println!("var value is : {}", var),
    }

    let var_from_match = match some_bool {
        true => 10,
        false => 20,
    };

    let var_from_match2 = match some_int {
        0 => 0,
        1 | 2 => 100,
        _ => 200,
    };
}
