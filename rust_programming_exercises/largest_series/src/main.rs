use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", lsp("23456783", 3).unwrap());
}

fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    // if span == 0 {
    //     Ok(1)
    // } else {
    //     let results = string_digits
    //         .chars()
    //         .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
    //         .collect::<Result<Vec<u32>, Error>>()?;
    //     results
    //         .windows(span)
    //         .map(|w| w.iter().map(|&n| n as u64).product())
    //         .max()
    //         .ok_or(Error::SpanTooLong)
    // }

    if span == 0 {
        Ok(1)
    } else if span > string_digits.len() {
        Err(Error::SpanTooLong)
    } else {
        let mut d = VecDeque::new();
        let mut prod: u64 = 1;
        let mut max: u64 = 0;
        let mut zeroes: usize = 0;

        for c in string_digits.chars() {
            let val = c.to_digit(10);
            if val.is_none() {
                return Err(Error::InvalidDigit(c));
            }

            let n = val.unwrap() as u64;
            d.push_front(n);

            if n == 0 {
                zeroes += 1;
            } else {
                prod = prod * n;
            }

            if d.len() > span {
                let prev = d.pop_back().unwrap();
                if prev == 0 {
                    zeroes -= 1;
                } else {
                    prod = prod / prev;
                }
            }
            if d.len() == span && prod > max && zeroes == 0 {
                max = prod;
            }
        }
        Ok(max)
    }
}
