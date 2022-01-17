#[allow(unused_variables)]
fn main() {
    /*
        let var_a = String::from("Howdy");
        let var_b = var_a;
        println!("{}", var_a); // Error
        println!("{}", var_b);
    */

    // STACK
    // - Fast memory creation and retrieval... speed, Speed, SPEED!
    // - Memory is automatically recaptured by the program after variables go out of scope.
    // - Is the default in Rust
    // - Fixed size variables... Collections CANNOT be stack based (and Strings are a collection of u8's)
    /*
    let stack_i8: i8 = 10;
     let stack_f32: f32 = 20.0;
     let stack_bool: bool = true;
     let stack_char: char = 'a';
     */
    /*
    if stack_i8 == 3 {
        let inside_scope = 9;
        println!("{}", inside_scope);
    }
    println!("{}", inside_scope); // Error
     */

    // HEAP
    // - Flexibility
    // - Memory that can grow in size (Vector, HashMap, String etc)
    // - Runtime performance cost (speed)
    // - Memory that can live beyond the scope that created it
    // - Memory is automatically recaptured when the last OWNER goes out of scope

    /*
    let heap_vector: Vec<i8> = Vec::new(); // vec![5, 2];
    let heap_string: String = String::from("Howdy");
    let heap_i8: Box<i8> = Box::new(30);

    let stack_i8_2 = stack_i8; // stack_i8 value copy to stack_i8_2
    println!("{}", stack_i8);
    println!("{}", stack_i8_2);
    */

    /*
    let heap_i8_2 = heap_i8; // heap_i8 value move to heap_i8_2 variable, now heap_i8 is no longer valid
    println!("{}", heap_i8); // Error
    println!("{}", heap_i8_2);
    */

    /*
    let heap_i8_2 = heap_i8.clone(); // copy heap_i8 to heap_i8_2 variable
    println!("{}", heap_i8);
    println!("{}", heap_i8_2);
     */

    /*
    let heap_i8_2 = &heap_i8; // heap_i8_2 has reference to heap_i8 value
    println!("{}", heap_i8);
    println!("{}", heap_i8_2);
    */

    /*
    let stack_f64: f64 = 1.0;
    let heap_f64: Box<f64> = Box::new(2.0);

    stack_procedure(stack_f64);
    println!("In main stack {}", stack_f64);
    /*
    heap_procedure(heap_f64); // heap_64 value move to this function, now heap_f64 no longer valid
    heap_procedure(heap_f64.clone()); // cloning variable performance issues
    println!("In main heap {}", heap_f64); // heap_f64 no longer valid
     */
    heap_procedure(&heap_f64);
    println!("In main heap {}", heap_f64);
     */

    /*
    ==============
    Strings
    ==============
     */
    /*
    let some_string: String = String::from("Howdy"); // Strings are always on the heap
    let some_str: &str = "Partner"; // &str is a pointer... to either stack or heap
    some_procedure(&some_string, some_str);
    println!("{} {}", some_string, some_str);
    */

    // any no of immutable references exists
    /*
     let var_a = String::from("Howdy!");
     let var_b = &var_a;
     let var_c = &var_a;
     println!("{} {} {}", var_a, var_b, var_c);
    */

    /*
    let var_a = String::from("Howdy!");
    let mut var_b = &var_a;
    var_b.push('b'); // Error
    let var_c = &var_a;
    println!("{} {} {}", var_a, var_b, var_c);
     */

    /*
     let mut var_a = String::from("Howdy!");
     let var_b = &var_a;
     let var_c = &var_a;
     var_a.push('a'); // Error
     println!("{} {} {}", var_a, var_b, var_c);
    */
    /*
    let mut var_a = String::from("Howdy!");
    let var_b = &var_a;
    let var_c = &var_a;
    println!("{} {} {}", var_a, var_b, var_c);
    var_a.push('a'); // Valid because var_b, var_c reference no longer exist
     */

    /*
    let mut var_a = String::from("Howdy!");
    var_a.push('a'); // Valid because var_b, var_c reference are not exist yet
    let var_b = &var_a;
    let var_c = &var_a;
    println!("{} {} {}", var_a, var_b, var_c);
    */

    /*
    let mut var_a = String::from("Howdy!");
    let var_b = &var_a;
    let var_c = &var_a;
    println!("{} {} {}", var_a, var_b, var_c);
    var_a.push('a'); // Error because var_b still reference var_a after this line
    println!("{}", var_b);
     */

    /*
    let var_a = String::from("Howdy");
    let var_b = String::from("Partner");
    let mass_data: Vec<&String> = vec![&var_a, &var_b]; //COULD BE MILLIONS OF VALUES
    let mass_data: Vec<&str> = vec![&var_a, &var_b]; //COULD BE MILLIONS OF VALUES

    println!("{}", heavy_calcs(&mass_data));
    println!("{} {} {:?}", var_a, var_b, mass_data);
     */

    /*
    ================
    Structs
    ================
    */
    let mut var_1 = PrinceStruct { a: 9, b: 10.1 };
    // some_procedure(var_1.clone());
    // println!("{:?}", var_1);
    // some_procedure(var_1);
    // println!("{:?}", var_1); // Error
    some_procedure(&mut var_1);
    println!("{:?}", var_1);
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct PrinceStruct {
    a: i32,
    b: f64,
}

fn some_procedure(param_a: &mut PrinceStruct) {
    param_a.a = 15;
    println!("{:?}", param_a);
}

/*
fn heavy_calcs(_param: &Vec<&str>) -> i64 {
    // Some heavy duty calcs performed here that utilize available cores of my comp
    10
}
 */

/*
 fn some_procedure(param_a: &String, param_b: &str) {
    println!("{} {}", param_a, param_b);
}
 */

/*
fn stack_procedure(mut param: f64) {
    param += 9.0;
    println!("In stack_procedure with param {}", param);
}

fn heap_procedure(param: &Box<f64>) {
    println!("In heap_procedure with param {}", param);
}
 */
/*
fn some_procedure() {
    let inside_procedure = 9;
}
 */
