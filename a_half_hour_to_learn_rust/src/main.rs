use std::{
    cmp::max,
    fmt::{Debug, Display},
};
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let x;
    x = 42;

    let x = 42;

    let x: i64;
    x = 123;
    let x: i8 = 42;

    let _ = 323;
    let _x = 23;

    let x = 13;
    let x = x + 3;

    let pair = ('a', 17);
    pair.0;
    pair.1;

    let pair: (char, i32) = ('b', 1);
    let (some_char, some_int) = ('c', 3);

    let slice = String::from("HelloWorld");
    let (left, right) = slice.split_at(5);
    println!("{} {}", left, right);
    let (_, right) = slice.split_at(5);

    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);
    println!("{}", x);

    greet();
    println!("{}", fair_dice_roll(true));

    let x = "out";
    {
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);

    let x = {
        let x = 5;
        x
    };

    let nick = "Nick";
    println!("{}", nick.len());

    let least = std::cmp::min(3, 8);
    let more = max(3, 8);
    println!("Min of 3 and 8 is {least}");
    println!("Max of 3 and 8 is {more}");

    let x = "amos".len();
    let x = str::len("amos");

    let v: Vec<i8> = Vec::new();

    let v: Vec<i32> = std::vec::Vec::new();

    let v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2 = Vec2 { x: 2.0, y: 4.0 };
    let v3 = Vec2 { x: 14.0, ..v2 };
    let v4 = Vec2 { ..v3 };

    let v = Vec2 { x: 3.0, y: 6.8 };
    let Vec2 { x, y } = v;
    let Vec2 { x, .. } = v;

    let one = Number {
        odd: true,
        value: 1,
    };
    let two = Number {
        odd: false,
        value: 2,
    };
    print_number(one);
    print_number(two);

    let minus_two = Number {
        odd: false,
        value: -2,
    };
    println!("{}", minus_two.is_strictly_positive());

    let mut n = Number {
        odd: true,
        value: 17,
    };
    n.value = 19;

    let minus_two = Number {
        odd: false,
        value: -2,
    };
    println!("{}", minus_two.is_strictly_negative());

    let n: i32 = -4;
    println!("{}", n.is_strictly_negative());

    let n = Number {
        odd: true,
        value: 987,
    };
    let m = -n;
    println!("{}", m.value);

    let n = Number {
        odd: true,
        value: 987,
    };
    let m = n.clone();
    let m = std::clone::Clone::clone(&n);

    let n = Number {
        odd: true,
        value: 987,
    };
    let m = n.clone();
    let o = n.clone();

    let m = n;
    let o = n;

    foobar(1, 2);
    foobar1(1, 2);
    compare("tea", "coffee");

    use std::any::type_name;
    println!("{}", type_name::<i32>());
    println!("{}", type_name::<(char, i32)>());

    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: false };
    print_type_name(&p1);
    print_type_name(&p2);
}

#[allow(dead_code)]
struct Pair<T> {
    a: T,
    b: T,
}
fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn foobar<L: Display, R: Display>(left: L, right: R) {
    println!("{} {}", left, right)
}

fn foobar1<L, R>(left: L, right: R)
where
    L: Display + Debug,
    R: Display,
{
    println!("{} {}", left, right)
}

fn compare<T>(left: T, right: T)
where
    T: Debug + PartialEq,
{
    println!(
        "{:?} {} {:?}",
        left,
        if left == right { "==" } else { "!=" },
        right
    );
}

fn print_number(n: Number) {
    // if let Number { odd: true, value } = n {
    //     println!("Odd number: {}", value)
    // } else if let Number { odd: false, value } = n {
    //     println!("Even number: {}", value)
    // }

    // match n {
    //     Number { odd: true, value } => println!("Odd number: {}", value),
    //     Number { odd: false, value } => println!("Even number: {}", value),
    // }

    // match n {
    //     Number { value: 1, .. } => println!("One"),
    //     Number { value: 2, .. } => println!("Two"),
    //     Number { value, .. } => println!("{}", value),
    // }

    match n.value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("{}", n.value),
    }
}

trait Signed {
    fn is_strictly_negative(self) -> bool;
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Number {
    odd: bool,
    value: i32,
}

// impl std::clone::Clone for Number {
//     fn clone(&self) -> Self {
//         Self { ..*self }
//     }
// }

// impl std::marker::Copy for Number{}

impl std::ops::Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        Number {
            value: -self.value,
            odd: self.odd,
        }
    }
}

impl Signed for Number {
    fn is_strictly_negative(self) -> bool {
        self.value < 0
    }
}

impl Signed for i32 {
    fn is_strictly_negative(self) -> bool {
        self < 0
    }
}

impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}

#[allow(dead_code)]
struct Vec2 {
    x: f64,
    y: f64,
}

fn greet() {
    println!("Hi there!");
}

fn fair_dice_roll(feeling_lucky: bool) -> i32 {
    // if feeling_lucky {
    //     6
    // } else {
    //     4
    // }
    match feeling_lucky {
        true => 6,
        false => 4,
    }
}
