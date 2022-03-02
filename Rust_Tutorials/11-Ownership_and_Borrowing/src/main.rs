#[allow(unused_variables)]
fn main() {
    println!("-----------Ownership and Borrowing-----------");

    // 1.first_compile_error
    // first_compile_error();

    // 2.stack_and_heap
    // stack_and_heap();

    // 3.passing_value_to_procedure
    // passing_value_to_procedure();

    // 4.string_vs_str_slices
    // string_vs_str_slices();

    // 5.heap_data_can_have_one_owner_at_a_time
    heap_data_can_have_one_owner_at_a_time();
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn first_compile_error() {
    let var_a = String::from("Howdy");
    let var_b = var_a;
    // println!("{}", var_a); // Error
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn stack_and_heap() {
    // STACk
    // - Fast memory creation and retrieval... speed, Speed, SPEED!
    // - Memory is automatically recaptured by the program after variables go out of scope
    // - Is the default is Rust
    // - Fixed size variables.... Collections CANNOT be stack based (and Strings are a collection of u8's)
    let stack_i8: i8 = 10;
    let stack_f32: f32 = 20.;
    let stack_bool: bool = true;
    let stack_char: char = 'a';

    if stack_i8 == 3 {
        let inside_scope = 9;
        println!("{}", inside_scope);
    }
    // println!("{}", inside_scope); // Error, cannot find value `inside_scope` in this scope
    fn some_procedure() {
        let inside_procedure = 9.;
    }
    // println!("{}", inside_procedure); // Error, cannot find value `inside_procedure` in this scope

    // HEAP
    // - Flexibility
    // - Memory that can grow in size (Vector, HashMap, String, etc)
    // - Runtime performance cost (speed)
    // - Memory that can live beyond the scope that created it
    // - Memory is automatically recaptured when the last OWNER goes out of scope
    let heap_vector: Vec<i8> = Vec::new(); // vec![5,2]
    let heap_string: String = String::from("Howdy");
    let heap_i8: Box<i8> = Box::new(30);

    let stack_i8_2 = stack_i8;
    println!("{}", stack_i8);
    println!("{}", stack_i8_2);

    // First time error
    // let heap_i8_2 = heap_i8; // `heap_i8` move here
    // println!("{}", heap_i8); // Error, heap_i8 no longer point to any thing
    // println!("{}", heap_i8_2);

    // Solutions
    // 1. First
    let heap_i8_2 = &heap_i8; // now `heap_i8_2` refer to `heap_i8`, `heap_i8` is not moved here
    println!("{}", heap_i8); // heap_i8 is valid
    println!("{}", heap_i8_2);

    // 2. Second
    let heap_i8_2 = heap_i8.clone(); // now `heap_i8_2` is copy of heap_i8, both are different variable, Cloning is expensive
    println!("{}", heap_i8); // heap_i8 is valid
    println!("{}", heap_i8_2);
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn passing_value_to_procedure() {
    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    stack_procedure(stack_f64);
    println!("In main stack {}", stack_f64);

    // Problem
    // heap_procedure(heap_f64); // heap_f64 moved here
    // println!("In main heap {}", heap_f64); // Error, borrow of moved value: `heap_f64`

    // Solutions
    // 1. Cloning, both variables are different
    // heap_procedure(heap_f64.clone()); // Passing copy of heap_f64 value
    // println!("In main heap {}", heap_f64);

    // 2. Returning from procedure, but this solution sucks
    // let heap1_f64: Box<f64> = Box::new(2.);
    // heap1_f64 = heap_function(heap1_f64); // `heap1_f64` moved here
    // println!("In main heap {}", heap1_f64);

    // 3. passing reference of heap_f64,
    // One, and only one, owner of a piece of memory at a time
    heap_procedure_borrow(&heap_f64); // `heap_procedure_borrow` borrow `heap_f64`
    println!("In main heap {}", heap_f64);

    // Procedures Definition
    fn stack_procedure(mut param: f64) {
        param += 9.;
        println!("In stack_procedure with param {}", param);
    }

    fn heap_procedure(param: Box<f64>) {
        println!("In heap_procedure with param {}", param);
    }

    fn heap_procedure_borrow(param: &Box<f64>) {
        println!("In heap_procedure with param {}", param);
    }

    fn heap_function(param: Box<f64>) -> Box<f64> {
        println!("In heap_procedure with param {}", param);
        param
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn string_vs_str_slices() {
    let some_string: String = String::from("Howdy"); // Strings are always on the heap
    let some_str: &str = "Partner"; // &str is a pointer... to either stack or heap

    // Problem
    // some_procedure(some_string, some_str); // `some_string` moved to `some_procedure`
    // println!("{} {}", some_string, some_str); // Error, borrow of moved value: `some_string`

    // Solutions
    some_procedure1(&some_string, some_str); // passing some_string reference
    println!("{} {}", some_string, some_str); // some_string is still valid

    // Procedure Definition
    fn some_procedure(param_a: String, param_b: &str) {
        println!("{} {}", param_a, param_b);
    }
    fn some_procedure1(param_a: &String, param_b: &str) {
        println!("{} {}", param_a, param_b);
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn heap_data_can_have_one_owner_at_a_time() {
    // 1.first
    // let var_a = String::from("Howdy");
    // let var_b = &var_a; // var_b reference var_a data, var_a remains owner
    // let var_c = &var_a; // var_c reference var_a data, var_a remains owner

    // println!("{} {} {}", var_a, var_b, var_c);

    // 2.second
    // let mut var_a = String::from("Howdy!");
    // var_a.push('a'); // it is valid

    // let var_b = &var_a; // var_b reference var_a data, var_a remains owner
    // let var_c = &var_a; // var_c reference var_a data, var_a remains owner

    // // var_a.push('a'); // Error, cannot borrow `var_a` as mutable because it is also borrowed as immutable
    // println!("{} {} {}", var_a, var_b, var_c);
    // var_a.push('a'); // No, Error, because var_b, var_c no longer valid or in use

    // 3.Third
    // let var_a = String::from("Howdy");
    // let var_b = String::from("Partner");

    // let mass_data: Vec<&str> = vec![&var_a, &var_b]; // COULD BE MILLIONS OF VALUES

    // println!("{}", heavy_calcs(&mass_data));
    // println!("{} {} {:?}", var_a, var_b, mass_data);

    // fn heavy_calcs(param: &Vec<&str>) -> i64 {
    //     // Some heavy duty calcs performed here that utilize available cores of my computer

    //     10
    // }

    // 4.Four
    // #[derive(Debug)]
    // struct PrinceStruct {
    //     a: i32,
    //     b: f64,
    // }

    // let var_1 = PrinceStruct { a: 9, b: 10. };
    // some_procedure(var_1); // var_1 moved here
    // println!("{:?}", var_1); // Error, borrow of moved value: `var_1`

    // fn some_procedure(param_a: PrinceStruct) {
    //     println!("{:?}", param_a);
    // }

    // 5.Five
    #[derive(Debug)]
    struct PrinceStruct {
        a: i32,
        b: f64,
    }

    let mut var_1 = PrinceStruct { a: 9, b: 10. };
    some_procedure(&mut var_1);
    println!("{:?}", var_1);

    fn some_procedure(param_a: &mut PrinceStruct) {
        param_a.a = 15;
        println!("{:?}", param_a);
    }
}
