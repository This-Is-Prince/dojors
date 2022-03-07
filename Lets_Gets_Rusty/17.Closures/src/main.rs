use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 1.First Time Function
/* #[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        )
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
} */

// 2.Second Time Function
/* #[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result)
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
} */

// 3.Third Time Function
/* #[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // Error, mismatched types

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity))
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
} */

// 3.Third Time Function
/*struct CacheFn<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> CacheFn<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
#[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    let mut cache_result = CacheFn::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cache_result.value(intensity));
        println!("Next, do {} situps!", cache_result.value(intensity))
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cache_result.value(intensity));
        }
    }
} */

// 4.fourth Time Function
struct CacheFn<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> CacheFn<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if let Some(v) = self.value.get(&arg) {
            return *v;
        }
        self.value.insert(arg, (self.calculation)(arg));
        self.value[&arg]
    }
}
#[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    let mut cache_result = CacheFn::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cache_result.value(intensity));
        println!("Next, do {} situps!", cache_result.value(intensity))
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cache_result.value(intensity));
        }
    }
}

fn first() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_intensity, simulated_random_number);
}

fn second() {
    let x = 4;
    let equal_to_x = |z| z == x;

    /*
    fn equal_to_x(z: i32) -> bool {
        z == x // ERROR, can't capture dynamic environment in a fn item
    }
    */

    let y = 4;
    assert!(equal_to_x(y));
}

fn third() {
    let x = vec![1, 2, 3];

    let equal_to_x = |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

// fn fourth() {
//     let x = vec![1, 2, 3];

//     let equal_to_x = move |z| z == x;

//     println!("can't use x here: {:?}", x); // ERROR, borrow of moved value: `x` 

//     let y = vec![1, 2, 3];

//     assert!(equal_to_x(y));
// }

fn main() {
    println!("----------Closures In Rust----------");

    first();
    second();
    third();
    // fourth();
}
