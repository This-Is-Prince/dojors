#[allow(unused_variables)]
fn main() {
    println!("------Lifetimes------");

    // 1.first
    // first();

    // 2.second
    // second();

    // 3.static_lifetimes
    // static_lifetimes();

    // 4.some_examples
    some_examples();
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn first() {
    let a;
    {
        let b = String::from("Howdy");
        a = b;
        // a = &b; // Error, `b` does not live long enough
    }
    println!("{}", a);
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn second() {
    // fn get_int_ref() -> &i32 //Error, missing lifetime specifier
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
    // let greater = some_func(&a, &SOME_CONST_B); // Error, `a` does not live long enough
    // println!("{}", greater);
    // fn some_func(param_1: &'static str, param_2: &'static str) -> &'static str {
    //     if param_1 > param_2 {
    //         param_1
    //     } else {
    //         param_2
    //     }
    // }

    let a = String::from("a");
    let greater = some_func(&a, &SOME_CONST_B);
    println!("{}", greater);
    fn some_func<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
        if param_1 > param_2 {
            param_1
        } else {
            param_2
        }
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn return_one_reference_from_multiple() {
    let some_int_var1 = 10;
    let some_int_var2 = 10;
    let result_ref = get_inf_ref(&some_int_var1, &some_int_var2);
    println!("{}", result_ref);

    fn get_inf_ref<'a, 'b: 'a>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
        if param_1 > param_2 {
            param_1
        } else {
            param_2
        }
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn some_examples() {
    fn test_1(param_1: Vec<f64>) -> Vec<f64> /*Lifetimes don't apply because there are no reference inputs or output */
    {
        param_1
    }

    fn test_2<'a>(param_1: &'a Vec<f64>) -> Vec<f64> /*Lifetimes aren't an issue because there is no reference output*/
    {
        param_1.clone()
    }

    // fn test_3<'a>(param_1: Vec<f64>) -> &'a Vec<f64> /*Lifetimes don't apply because there are no reference inputs */
    // {
    //     &param_1
    // }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn lifetimes_with_generics() {
    // fn get_smaller<'a, T: std::cmp::PartialOrd>(param_1: &'a T, param_2: &'a T) -> &'a T {
    //     if param_1 < param_2 {
    //         param_1
    //     } else {
    //         param_2
    //     }
    // }

    fn get_smaller<'a, T>(param_1: &'a T, param_2: &'a T) -> &'a T
    where
        T: std::cmp::PartialOrd,
    {
        if param_1 < param_2 {
            param_1
        } else {
            param_2
        }
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn struct_with_lifetimes() {
    struct PrinceStruct<'a, 'b> {
        some_data: Vec<i32>,
        some_reference_data1: &'a Vec<i32>,
        some_reference_data2: &'b Vec<i32>,
    }
}
