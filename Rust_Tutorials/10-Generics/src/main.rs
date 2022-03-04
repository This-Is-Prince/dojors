#[allow(dead_code)]
#[allow(unused_variables)]
struct Point<T, U> {
    x: T,
    y: U,
}

#[allow(dead_code)]
#[allow(unused_variables)]
enum SomeEnum<T, U> {
    OptionA(T),
    OptionB(U),
    OptionC,
}

trait SomeCustomTrait {
    fn blah_blah(&self, a: &str, b: &str) -> String;
}

#[allow(dead_code)]
#[allow(unused_variables)]
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
    println!("{:?}", some_var);
    some_var.blah_blah("First", "Second")
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    println!("----------Generics----------");

    let a = Point {
        x: 100 as u8,
        y: -100. as f32,
    };

    // Enums can also be Generics
    let some_data: SomeEnum<f64, i32> = SomeEnum::OptionA(34.2);
    let some_data2: SomeEnum<u8, i64> = SomeEnum::OptionA(34);

    match some_data {
        SomeEnum::OptionA(a) => println!("OptionA contains {}", a),
        SomeEnum::OptionB(b) => println!("OptionB contains {}", b),
        SomeEnum::OptionC => println!("OptionC contains"),
    }

    prince_func(1, 2.0);
    prince_func(100 as u8, "hello");

    prince_func2(10000, 1.5);

    let test = PrinceStruct { something: 1000 };
    let result = do_this(&test);
    println!("{:?}", result);

    let test_i32 = 18;
    let result_2 = do_this(&test_i32);
    println!("{:?}", result_2);
}

fn prince_func<T: std::ops::Add<Output = T> + std::fmt::Debug, U: std::fmt::Debug>(
    input_a: T,
    input_b: U,
) -> T {
    println!("input_a has {:?}, input_b has {:?}", input_a, input_b);
    input_a
}

fn prince_func2<T, U>(input_a: T, input_b: U) -> T
where
    T: std::ops::Add<Output = T> + std::fmt::Debug,
    U: std::fmt::Debug,
{
    println!("input_a has {:?}, input_b has {:?}", input_a, input_b);
    input_a
}
