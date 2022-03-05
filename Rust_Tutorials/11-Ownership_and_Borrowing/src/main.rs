#[allow(unused_variables)]
fn main() {
    println!("----------Ownership and Borrowing----------");

    // 1. first compiler error
    first_compile_error();

    // 2.stack and heap
    stack_and_heap();

    // 3.passing_value_to_procedure
    passing_value_to_procedure();

    // 4.string vs str slices
    string_vs_str_slices();

    // 5.heap_data_can_have one owner at a time
    heap_data_can_have_one_owner_at_a_time();
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn first_compile_error() {
    let var_a = String::from("Prince");
    let var_b = var_a; // var_a moved to var_b, after this line var_a is not valid.
                       // println!("{}", var_a); // ERROR, borrow of moved value: `var_a`
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn stack_and_heap() {
    // STACK
    // - Fast memory creation and retrieval... speed, SPEED, SPEED!
    // - Memory is automatically recaptured by the program after variables go out of scope
    // - Is the default is Rust
    // - Fixed size variables.... Collections CANNOT be stack based (and Strings are a collection of u8's)
    let stack_i8: i8 = 10;
    let stack_f32: f32 = 20.;
    let stack_bool: bool = true;
    let stack_char: char = 'a';

    if stack_i8 == 3 {
        let inside_scope = 9;
        println!(
            "inside_scope variable in inside of if scope {}",
            inside_scope
        );
    }
    // println!(
    //     "inside_scope variable in outside of if scope {}",
    //     inside_scope
    // ); // ERROR, cannot find value `inside_scope` in this scope

    fn some_procedure() {
        let inside_procedure = 9.;
    }
    // println!(
    //     "inside_procedure variable in outside of some_procedure {}",
    //     inside_procedure
    // ); // ERROR, cannot find value `inside_procedure` in this scope

    // HEAP
    // - Flexibility
    // - Memory that can grow in size (Vector, HashMap, String etc)
    // - Runtime performance cost (speed)
    // - Memory that can live beyond the scope that created it.
    // - Memory is automatically recaptured when the last ##OWNER## goes out of scope
    let heap_vector: Vec<i8> = Vec::new(); // vec![5,2]
    let heap_string: String = String::from("Prince");
    let heap_i8: Box<i8> = Box::new(30);

    let stack_i8_2 = stack_i8;
    println!("{}", stack_i8);
    println!("{}", stack_i8_2);

    // First Time ERROR
    // let heap_i8_2 = heap_i8;
    // println!("{}", heap_i8); // ERROR, borrow of moved value: `heap_i8`
    // println!("{}", heap_i8_2);

    // Solutions
    // 1. FIRST
    let heap_i8_2 = &heap_i8; // now `heap_i8_2` refer to `heap_i8`, `heap_i8` is not moved here
    println!("{}", heap_i8); // heap_i8 is valid
    println!("{}", heap_i8_2);

    // 2. SECOND
    let heap_i8_2 = heap_i8.clone(); // now `heap_i8_2` is copy of `heap_i8`, both are different variable, Cloning is expensive
    println!("{}", heap_i8); // heap_i8 is valid
    println!("{}", heap_i8_2);
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn passing_value_to_procedure() {
    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    stack_procedure(stack_f64);
    println!("In Main stack {}", stack_f64);

    // PROBLEM
    // heap_procedure(heap_f64);
    // println!("In Main Heap {}", heap_f64); // ERROR, borrow of moved value: `heap_f64`

    // Solutions
    // 1. Cloning, both variables are different
    // heap_procedure(heap_f64.clone()); // Passing copy of heap_f64 value
    // println!("In main heap {}", heap_f64);

    // 2. Returning from procedure, but this solution sucks
    // let heap1_f64: Box<f64> = Box::new(2.);
    // let heap1_f64 = heap_function(heap1_f64); // `heap1_f64` moved here
    // println!("In main heap {}", heap1_f64);

    // 3. passing reference of heap_f64,
    // One, and only one, owner of a piece of memory at a time.
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

    fn heap_function(param: Box<f64>) -> Box<f64> {
        println!("In heap_function with param {}", param);
        param
    }

    fn heap_procedure_borrow(param: &Box<f64>) {
        println!("In heap_procedure borrow with param {}", param);
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn string_vs_str_slices() {
    let some_string: String = String::from("Prince"); // Strings are always on the heap.
    let some_str: &str = "Partner"; // &str is a pointer... to either stack or heap.

    // Problem
    // some_procedure(some_string, some_str); // `some_string` moved to `some_procedure`
    // println!("{} {}", some_string, some_str); // ERROR, borrow of moved value: `some_string`

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
    let var_a = String::from("Prince");
    let var_b = &var_a; // var_b reference var_a data, var_a remains owner
    let var_c = &var_a; // var_c reference var_a data, var_a remains owner

    println!("{} {} {}", var_a, var_b, var_c);

    // 2.second
    let mut var_a = String::from("Prince");
    var_a.push('a'); // it is valid

    let var_b = &var_a; // var_b reference var_a data, var_a remains owner
    let var_c = &var_a; // var_c reference var_a data, var_a remains owner

    // var_a.push('a'); // ERROR, cannot borrow `var_a` as mutable because it is also borrowed as immutable
    println!("{} {} {}", var_a, var_b, var_c);
    var_a.push('a'); // No, Error, because var_b, var_c no longer valid or in use.

    // 3.Third
    let var_a = String::from("Prince");
    let var_b = String::from("Kumar");

    let mass_data: Vec<&str> = vec![&var_a, &var_b]; // COULD BE MILLIONS OF VALUES

    println!("{}", heavy_calcs(&mass_data));
    println!("{} {} {:?}", var_a, var_b, mass_data);

    fn heavy_calcs(param: &Vec<&str>) -> i64 {
        // Some heavy duty calcs performed here that utilize available cores of my computer
        10
    }

    // 4.Four
    #[derive(Debug)]
    struct PrinceStruct {
        a: i32,
        b: f64,
    }

    let var_1 = PrinceStruct { a: 9, b: 10. };
    some_procedure(var_1); // var_1 moved here
                           // println!("{:?}", var_1); // ERROR, borrow of moved value: `var_1`

    fn some_procedure(param_a: PrinceStruct) {
        println!("{:?}", param_a);
    }

    // 5.Five
    #[derive(Debug)]
    struct PrinceStruct1 {
        a: i32,
        b: f64,
    }

    let mut var_a = PrinceStruct1 { a: 9, b: 10. };
    some_procedure1(&mut var_a);
    println!("{:?}", var_a);
    fn some_procedure1(param_a: &mut PrinceStruct1) {
        param_a.a = 15;
        println!("{:?}", param_a);
    }
}
