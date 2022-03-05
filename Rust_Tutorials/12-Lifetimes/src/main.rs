#[allow(unused_variables)]
fn main() {
    println!("-------------Lifetimes-------------");

    // 1.first
    first();
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn first() {
    let a;
    {
        let b = String::from("Prince");
        // a = &b; // ERROR, `b` does not live long enough
        a = b;
    }
    println!("{}", a);
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn second() {
    // fn get_int_ref() -> &i32 // ERROR, missing lifetime specifier
    // {
    //     let a = 1;
    //     &a
    // }

    let some_int_var = 10;
    let result_ref = get_int_ref(&some_int_var);
    println!("{}", result_ref);
    // fn get_int_ref(param_1: &i32) -> &i32 {
    //     param_1
    // }

    // OR

    fn get_int_ref<'a>(param_1: &'a i32) -> &'a i32 {
        param_1
    }

    fn some_func<'a, 'b>(param_1: &'a str, param_2: &'b str, param_3: Vec<u64>) -> &'a str {
        param_1
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
const SOME_CONST_A: &str = "I'm a constant!";
const SOME_CONST_B: &str = "I'm a constant, too!";
#[allow(unused_variables)]
#[allow(dead_code)]
fn static_lifetimes() {
    // let a = String::from("a");
    // let greater = some_func(&a, &SOME_CONST_B); // ERROR, `a` does not live long enough
    // fn some_func(param_1: &'static str, param_2: &'static str) -> &'static str {
    //     if param_1 > param_2 {
    //         param_1
    //     } else {
    //         param_2
    //     }
    // }

    let a = String::from("a");
    let greater = some_func(&a, &SOME_CONST_B);
    fn some_func<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
        if param_1 > param_2 {
            param_1
        } else {
            param_2
        }
    }
}
