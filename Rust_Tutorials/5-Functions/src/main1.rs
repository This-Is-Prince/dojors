#[allow(unused_variables)]
#[allow(dead_code)]
pub fn main1() {
    println!("------------Functions and Procedures------------");

    let returned_data = some_function(2.2, 50);
    println!("value of returned_data is :- {}", returned_data);

    some_procedure(2.5, 55);

    let param_a: i32 = 20;
    println!(
        "Before passing in the pass_by_immutable_value function value is :- {}",
        param_a
    );
    pass_by_immutable_value(param_a);
    println!(
        "After passing in the pass_by_immutable_value function value is :- {}",
        param_a
    );

    let param_a: i32 = 20;
    println!(
        "Before passing in the pass_by_mutable_value function value is :- {}",
        param_a
    );
    pass_by_mutable_value(param_a);
    println!(
        "After passing in the pass_by_mutable_value function value is :- {}",
        param_a
    );

    let param_a: i32 = 20;
    println!(
        "Before passing in the pass_by_immutable_reference function value is :- {}",
        param_a
    );
    pass_by_immutable_reference(&param_a);
    println!(
        "After passing in the pass_by_immutable_reference function value is :- {}",
        param_a
    );

    let mut param_a: i32 = 20;
    println!(
        "Before passing in the pass_by_mutable_reference function value is :- {}",
        param_a
    );
    pass_by_mutable_reference(&mut param_a);
    println!(
        "After passing in the pass_by_mutable_reference function value is :- {}",
        param_a
    );

    let mut param_a: i32 = 200;
    let mut param_b: i32 = 400;
    pass_by_mutable_reference_and1(&mut param_a, &mut param_b)
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn some_function(param_a: f32, param_b: i32) -> f32 {
    param_b as f32 * param_a
}

fn some_procedure(param_a: f32, param_b: i32) {
    println!("param_a is {1}, param_b is {0}", param_b, param_a);
}

fn pass_by_immutable_value(param_a: i32) {
    println!("This function is pass_by_immutable_value :- {}", param_a);
    // param_a = 0; // Error, cannot assign to immutable argument `param_a`
}

fn pass_by_mutable_value(mut param_a: i32) {
    println!("This function is pass_by_mutable_value :- {}", param_a);
    param_a = 0;
    println!(
        "after changing param_a, in pass_by_mutable_value :- {}",
        param_a
    );
}

fn pass_by_immutable_reference(param_a: &i32) {
    println!(
        "This function is pass_by_immutable_reference :- {}",
        param_a
    );
    // *param_a = 0; // ERROR, cannot assign to `*param_a`, which is behind a `&` reference
    println!(
        "after changing param_a, in pass_by_immutable_reference :- {}",
        param_a
    );
}

fn pass_by_mutable_reference(param_a: &mut i32) {
    println!(
        "This function is pass_by_immutable_reference :- {}",
        param_a
    );
    *param_a = 0;
    println!(
        "after changing param_a, in pass_by_immutable_reference :- {}",
        param_a
    );
}

fn pass_by_mutable_reference_and1<'a>(mut param_a: &'a mut i32, param_b: &'a mut i32) {
    println!(
        "This function is pass_by_immutable_reference :- {}",
        param_a
    );
    *param_a = 0;
    println!(
        "after changing param_a, in pass_by_immutable_reference :- {}",
        param_a
    );
    param_a = param_b;
    println!("after assigning param_b to param_a :- {}", param_a);
}
