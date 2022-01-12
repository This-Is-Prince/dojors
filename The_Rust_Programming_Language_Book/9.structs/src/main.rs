struct User {
    name: String,
    email: String,
    age: u32,
}

fn main() {
    let user_1 = User {
        name: String::from("Prince"),
        email: String::from("prince@gmail.com"),
        age: 21,
    };
    println!("The value of user_1.name is : {}", user_1.name);
    println!("The value of user_1.name is : {}", user_1.email);
    println!("The value of user_1.name is : {}", user_1.age);

    let mut user_1 = User {
        name: String::from("Prince"),
        email: String::from("prince@gmail.com"),
        age: 21,
    };
    println!("The value of user_1.name is : {}", user_1.name);
    println!("The value of user_1.name is : {}", user_1.email);
    println!("The value of user_1.name is : {}", user_1.age);
    user_1.email = String::from("princekumar@gmail.com");
    println!("The value of user_1.name is : {}", user_1.email);
    let mut user_2 = build_user(
        String::from("prince@gmail.com"),
        String::from("prince KUmar"),
    );
    let user_3 = User { age: 22, ..user_2 };
    user_2.email = String::from("PK@gmail.com");
    user_2.name = String::from("PK");
    println!("The value of user_1.name is : {}", user_2.email);
    println!("The value of user_1.name is : {}", user_2.name);
}

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        age: 20,
    }
}
