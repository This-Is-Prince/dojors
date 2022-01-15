const MEANING_OF_LIFE: u8 = 42; // no fixed address

static Z: i32 = 123;

static mut A: i32 = 12;

fn main() {
    println!("MEANING_OF_LIFE = {}", MEANING_OF_LIFE);
    println!("Z = {}", Z);
    some_fun();

    fn m() {
        const MEANING_OF_LIFE: u32 = 42999; // no fixed address
        println!("in m, MEANING_OF_LIFE = {}", MEANING_OF_LIFE);
        static mut B: i32 = 12;
        unsafe {
            println!("in m, B = {}", B);
        }
        println!("m");
    }
    m();
    // unsafe {
    //     println!("in m, B = {}", B); //Error
    // }
}
fn some_fun() {
    println!("in some_fun, MEANING_OF_LIFE = {}", MEANING_OF_LIFE);
    println!("in some_fun, Z = {}", Z);
    unsafe {
        println!("in some_fun, A = {}", A);
        A += 1;
    }
}
