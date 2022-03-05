#[allow(unused_imports)]
use std::cmp::PartialEq;
#[allow(unused_imports)]
use std::ops::{Add, AddAssign, Neg};

#[allow(dead_code)]
#[derive(Debug)]
struct Complex<T> {
    real: T,
    imag: T,
}

impl<T> Complex<T> {
    fn new(real: T, imag: T) -> Complex<T> {
        Complex::<T> { real, imag }
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;

    // a+b
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.real == rhs.real && self.imag == rhs.imag
    }
}

impl<T: Eq> Eq for Complex<T> where T: Eq {}

#[allow(unused_mut)]
#[allow(unused_variables)]
fn operator_overloading() {
    let mut a = Complex::new(1.7, 2.6);
    let mut b = Complex::new(3.3, 4.2);
    //     println!("{:?}", a);
    //     println!("{:?}", b);
    //     println!("{:?}", a + b);
    // a += b;
    // println!("{:?}", a);
    // println!("{:?}", -a);

    println!("{:?}", a != b);
}

fn main() {
    println!("----------Operator Overloading----------");

    operator_overloading();
}
