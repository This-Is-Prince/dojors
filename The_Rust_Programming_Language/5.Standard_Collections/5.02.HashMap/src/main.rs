use std::collections::HashMap;

fn main() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    // shapes.insert("square".to_string(), 5);
    shapes.insert("square".into(), 5);

    // println!("a square has {} sides", shapes["square"]);
    // println!("a square has {} sides", shapes["circle"]);

    // for (key, value) in &shapes {
    //     println!("{} : {}", key, value);
    // }

    println!("{:?}", shapes);
    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}
