// ---------------1-------------------
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// struct PrinceData<T> {
//     x: i32,
//     y: T,
//     z: T,
//     some_char: char,
// }

// fn main() {
//     let a = Point { x: 100, y: -1_f32 };
//     println!("x = {}, y = {}", a.x, a.y);

//     let b = Point {
//         x: 10.1,
//         y: -2.3_f32,
//     };
//     println!("x = {}, y = {}", b.x, b.y);
// }

// ---------------2-------------------
// enum SomeEnum<T> {
//     OptionA(T),
//     OptionB(T),
//     OptionC,
// }

// fn main() {
//     let some_data = SomeEnum::OptionA(34.2);

//     match some_data {
//         SomeEnum::OptionA(a) => println!("OptionA contains {}", a),
//         SomeEnum::OptionB(b) => println!("OptionB contains {}", b),
//         SomeEnum::OptionC => println!("Boring Option C :("),
//     }

//     let some_data2 = SomeEnum::OptionB('b');
//     let some_data3 = SomeEnum::OptionA(vec![1, 2, 3]);
// }

// ---------------3-------------------

// fn main() {
//     // let a = prince_func(4 as i8, 5);
//     let a = prince_func(4.6, 5.6);
//     println!("a has {}", a);
// }

// // fn prince_func<T>(input_a: T, input_b: T) -> T {
// //     input_a
// // }
// fn prince_func<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug>(
//     input_a: T,
//     input_b: T,
// ) -> T {
//     // input_a + input_b
//     println!("input_a has {:?}", input_a);
//     input_a - input_b
// }

// #[allow(dead_code)]
// fn prince_func2<T, E>(input_a: T, input_b: T, input_e: E) -> T
// where
//     T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug,
//     E: std::fmt::Debug,
// {
//     // input_a + input_b
//     println!("input_a has {:?}", input_a);
//     println!("input_e has {:?}", input_e);
//     input_a - input_b
// }

// ---------------4-------------------

trait SomeCustomTrait {
    fn blah_blah(&self, a: &str, b: &str) -> String;
}

#[allow(dead_code)]
fn do_this<T>(some_var: &T) -> String
where
    T: SomeCustomTrait + std::fmt::Debug,
{
    // Some complex logic
    println!("{:?}", some_var);
    some_var.blah_blah("first", "second")
}

#[allow(dead_code)]
fn do_this2(some_var: &dyn SomeCustomTrait) -> String {
    // Some complex logic..
    // println!("{:?}", some_var);
    some_var.blah_blah("first", "second")
}

fn main() {}
