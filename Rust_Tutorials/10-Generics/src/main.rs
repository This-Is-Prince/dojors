// struct PointI32 {
//     x: i32,
//     y: i32,
// }

// struct PointI64 {
//     x: f64,
//     y: f64,
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// enum SomeEnum<T> {
//     OptionA(T),
//     OptionB(T),
//     OptionC,
// }

trait SomeCustomTrait {
    fn blah_blah(&self, a: &str, b: &str) -> String;
}

#[derive(Debug)]
struct PrinceStruct {
    something: i32,
}

impl SomeCustomTrait for PrinceStruct {
    fn blah_blah(&self, a: &str, b: &str) -> String {
        self.something.to_string() + " - " + a + " - " + b
    }
}
impl SomeCustomTrait for i32 {
    fn blah_blah(&self, a: &str, b: &str) -> String {
        "i32".to_string() + " - " + a + " - " + b
    }
}

#[allow(dead_code)]
fn do_this<T>(some_var: &T) -> String
where
    T: SomeCustomTrait + std::fmt::Debug,
{
    // Some complex Logic
    println!("{:?}", some_var);
    some_var.blah_blah("first", "second")
}

// #[allow(unused_variables)]
// #[allow(dead_code)]
// fn do_this2(some_var: &dyn SomeCustomTrait) -> String {
//     // Some complex Logic
//     // println!("{:?}", some_var); // Error
//     "do_this2".to_string()
// }

fn main() {
    println!("-------------Generics-------------");

    // let a = Point { x: 100, y: -1 }; // T = i32 , U = i32
    // println!("x = {}, y = {}", a.x, a.y);

    // let b = Point { x: 10.1, y: -2.3 }; // T = f64 , U = f64
    // println!("x = {}, y = {}", b.x, b.y);

    // let a = Point { x: 100, y: -1.1 }; // T = i32 , U = f64
    // println!("x = {}, y = {}", a.x, a.y);

    // let b: Point<f64, i64> = Point { x: 10.5, y: -2 }; // T = f64 , U = i64
    // println!("x = {}, y = {}", b.x, b.y);

    // Enums can also be Generics
    // let some_data = SomeEnum::OptionA(34.2);

    // match some_data {
    //     SomeEnum::OptionA(a) => println!("OptionA contains {}", a),
    //     SomeEnum::OptionB(b) => println!("OptionA contains {}", b),
    //     SomeEnum::OptionC => println!("Boring option c : ("),
    // }

    // let some_data2 = SomeEnum::OptionB('b');
    // let some_data3 = SomeEnum::OptionA(vec![1, 2, 3]);

    // Function can also be Generics
    // let a: i32 = prince_func(4, 5);
    // println!("a has {}", a);
    // let a: u8 = prince_func(4 as u8, 255);
    // println!("a has {}", a);

    // SomeCustomTrait

    let test = PrinceStruct { something: 1000 };
    let result = do_this(&test);
    println!("{:?}", result);

    let test_i32 = 18;
    let result_2 = do_this(&test_i32);
    println!("{:?}", result_2);
}

// #[allow(unused_variables)]
// fn prince_func<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug>(
//     input_a: T,
//     input_b: T,
// ) -> T {
//     println!("input_a has {:?}", input_a);
//     input_a + input_b
// }

// #[allow(unused_variables)]
// #[allow(dead_code)]
// fn prince_func2<T, E>(input_a: T, input_b: T, input_e: E) -> T
// where
//     T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug,
//     E: std::fmt::Debug,
// {
//     println!("input_a has {:?}", input_a);
//     println!("input_b has {:?}", input_b);
//     input_a + input_b
// }
