#[allow(dead_code)]
#[allow(unused_variables)]
pub fn main1() {
    println!("----------Tuples----------");

    let some_tuple = (2, 3.4, 'a', "b".to_string(), (1.1, 'c', (500, false)));
    println!("some_tuple is :- {:?}", some_tuple);
    let some_bool = ((some_tuple.4).2).1;
    println!("some_bool is :- {}", some_bool);

    match some_tuple {
        (int, float, character, string, tuple) => {
            println!("int = {}", int);
            println!("float = {}", float);
            println!("character = {}", character);
            println!("string = {}", string);
            match tuple {
                (float, character, tuple) => {
                    println!("float = {}", float);
                    println!("character = {}", character);
                    match tuple {
                        (int, boolean) => {
                            println!("int = {}", int);
                            println!("boolean = {}", boolean);
                        }
                    }
                }
            }
        }
    }
}
