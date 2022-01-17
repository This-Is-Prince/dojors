// ------------1-------------
/*
fn main() {
    let x = 21;
    // this is closure
    let get_answer = |y| x + y;
    println!("{}", get_answer(x));
}
*/

// ------------2-------------
/* fn main() {
    // Regular function
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    // this is just a function pointer
    let _f = add;

    // Add function written as closure
    // let f = |x: i32, y: i32| {x + y};

    // simplified closure because of single expression
    // let f = |x: i32, y: i32| x + y;

    // Closure with inferred parameter types
    let f = |x, y| x + y;

    // Inline closure incl. function call
    println!("{}", (|x, y| x + y)(2, 2));

    let result = f(1, 2);
    println!("{}", result);
}
 */

// ------------3-------------

/* fn main() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    // fn(i32,i32)->i32 'fn' indicates function pointer
    fn calc_and_print(x: i32, y: i32, calculator: fn(i32, i32) -> i32) {
        let result = calculator(x, y);
        println!("{}", result);
    }

    calc_and_print(1, 2, add);
    // this is perfect fine as long as closure does't captured any environment variables,(any variable declare outside of the closure everything is ok).
    calc_and_print(1, 2, |x, y| x + y);

    // let z = 3;
    // The following closure does not work because it captures z.
    // Therefore, it cannot act as a function pointer.
    // The closure consists of the function plus the captured variable.
    // calc_and_print(1, 2, |x, y| x + y + z); // Error -> expected fn pointer, found closure
}
 */

// ------------4-------------
/* fn main() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    fn calc_and_print(x: i32, y: i32, calculator: Box<dyn Fn(i32, i32) -> i32 + '_>) {
        let result = calculator(x, y);
        println!("{}", result);
    }

    calc_and_print(1, 2, Box::new(add));
    calc_and_print(1, 2, Box::new(|x, y| x + y));

    // Now we can also pass a closure with capturing, to calc_and_print
    let z = 3;
    calc_and_print(1, 2, Box::new(|x, y| x + y + z));
}
 */

// ------------4 conceptually-------------
// what happening conceptually in 4
/* struct AdderClosure {
    z: i32,
}
trait MyAdder {
    fn add(&self, x: i32, y: i32) -> i32;
}

impl MyAdder for AdderClosure {
    fn add(&self, x: i32, y: i32) -> i32 {
        x + y + self.z
    }
}

fn main() {
    fn calc_and_print(x: i32, y: i32, calculator: Box<dyn MyAdder>) {
        let result = calculator.add(x, y);
        println!("{}", result);
    }
    let closure = AdderClosure { z: 3 };
    calc_and_print(1, 2, Box::new(closure));
}
 */

// ------------5-------------
fn main() {
    let mut result = 0;

    // Closure with mutable borrow
    let mut calc_result = |x, y| {
        result = x + y;
    };
    calc_result(1, 2);
    println!("{}", result);

    // Store closure in `FnMut` variable before calling it
    let mut result_calculator: Box<dyn FnMut(i32, i32)> = Box::new(|x, y| {
        result = x + y;
    });

    result_calculator(1, 2);
    drop(result_calculator);
    println!("{}", result);
}
