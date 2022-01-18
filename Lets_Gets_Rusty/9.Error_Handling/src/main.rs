#[allow(unused_imports)]
use std::fs::{self, File};
#[allow(unused_imports)]
use std::io::{self, ErrorKind, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    /* =============1============ */
    /*
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
    */

    /* =============2============ */
    /*
    let mut f = File::open("hello.txt")?;

    let mut s: String = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
     */

    /* =============3============ */
    /*
       let mut s: String = String::new();
       File::open("hello.txt")?.read_to_string(&mut s)?;
       Ok(s)
    */

    /* =============3============ */
    fs::read_to_string("hello.txt")
}

#[allow(unused_variables)]
fn main() -> Result<(), io::Error> {
    /*     // Panic Macro Crash program and display string
    panic!("crash and burn"); */

    /*    a(); */

    /*
    ===============
    Result Enum
    ===============
    */

    /*
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */

    /* =============1============ */
    /*     let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
     */

    /*
    /* =============2============ */
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        println!("Hello");
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
     */

    /*
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    */

    println!("name {}", read_username_from_file().unwrap());

    let f = File::open("hello.txt")?;
    Ok(())
}

/* fn a() {
    b();
}
fn b() {
    c(21);
}
fn c(num: i32) {
    if num == 22 {
        panic!("Don't  pass in 22!");
    }
}
 */
