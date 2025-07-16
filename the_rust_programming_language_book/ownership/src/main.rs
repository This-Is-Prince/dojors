fn main() {
   let s1 = "Hello";

   {
    let s2 = "World!";
    println!("{s2}")
   }

   println!("{s1}");
//    println!("{s2}"); // s2 scope ended

   let mut s = String::from("hello");
   s.push_str(", world!");
   println!("{s}");

   {
    let s3 = String::from("hello3");

    println!("{s3}");
   }

   // We can't access s3 here, because it already goes out of the scope


   let s4: String = String::from("Hello4");
   println!("{s4}");

   for v in s4.chars() {
    print!("{v} ");
   }

   let s5: String;

   let condition: bool = s4.len() == 6;
   if condition {
    s5 = s4;
   } else {
    s5 = "Hey".parse().unwrap();
   }

   println!("s5:- {s5}");

   // println!("s4:- {s4}"); // can't access s4 it goes out of the scope.

   let mut s6: String = String::from("s6");
   println!("{s6}");
   s6 = String::from("s61");
   println!("{s6}");

   s6 = change_s6_and_give_back(s6);
   println!("change and give back {s6}");

   change_s6(&mut s6);
   println!("change {s6}");

   move_occur(s6);

   // println!("again {s6}"); // can't use s6 because it moves to move_occur function.
}

fn change_s6(s: &mut String) {
    s.push_str("string");
}

fn move_occur(s: String) {
    println!("Move occur: {s}")
}

fn change_s6_and_give_back(mut s: String) -> String {
    s.push_str("  String  ");

    s
}