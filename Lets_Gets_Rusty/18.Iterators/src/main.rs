fn first() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }

    for value in v1 {
        println!("GOT: {}", value);
    }
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    // Methods with default implementations
}

fn second() {
    // iterator demonstration

    let v1 = vec![1, 2, 3];

    // .iter() returns immutable reference of value
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let mut v1 = vec![1, 2, 3];
    // .iter_mut() returns mutable reference of value
    let mut v1_iter = v1.iter_mut();

    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);

    println!("{:?}", v1);

    for x in v1.iter_mut() {
        *x += 1;
    }
    println!("{:?}", v1);
    let v1 = vec![1, 2, 3];
    println!("{:?}", v1);
    // .into_iter() returns a iterator which have ownership of v1 values and now v1 is not valid
    let mut v1_iter = v1.into_iter();
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);

    // println!("{:?}", v1); // ERROR, borrow of moved value: `v1`
}

fn sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total = v1_iter.sum::<i32>();

    assert_eq!(total, 6);
}

fn map() {
    let v1 = vec![1, 2, 3];
    // let mut iter = v1.iter().map(|x| x + 1);
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    for x in v1.iter().map(|x| x + 1) {
        print!("{}, ", x);
    }
    println!();
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn shoes() {
    let shoes = vec![
        Shoe {
            size: 8,
            style: String::from("Sports"),
        },
        Shoe {
            size: 7,
            style: String::from("Footwear"),
        },
        Shoe {
            size: 8,
            style: String::from("boot"),
        },
        Shoe {
            size: 9,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 8,
            style: String::from("PartyWear"),
        },
    ];
    println!("{:?}", shoes);
    let shoes = shoes_in_my_size(shoes, 8);
    println!("{:?}", shoes);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl std::iter::Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn count() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

fn main() {
    println!("---------Iterators---------");

    first();
    second();
    sum();
    map();
    shoes();
    count();
    using_other_iterator_trait_methods();
}
