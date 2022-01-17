#[allow(unused_variables)]
fn main() {
    // v is on stack but right side of equal to is on heap (means vec data is on heap v is just a pointer to that data).
    let v = vec![1, 2, 3];

    /*
    let v2 = v; // v value move to v2
    println!("{:?}", v); // Error v is no longer exist
    */

    /*
    let foo = |v: Vec<i32>| ();
    foo(v); // v move to foo
    println!("{:?}", v); // Error v is no longer exist
     */

    /*
    let u = 1;
    let u2 = u; // u value copy to u2
    println!("u = {}", u); // no error
    */

    let u = Box::new(1);
    let u2 = u; // u value move to u2 (no copy because u value is on heap)
                // println!("u = {}", u); // Error u is no longer exist

    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };
    let vv = print_vector(v);
    // println!("{:?}", v); // Error v is no longer exist
    println!("{:?}", vv);
}
