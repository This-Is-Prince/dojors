/* ================1=============== */
use std::fmt::Debug;
use std::fmt::Display;

// we can implement trait on type which implement another trait
/* impl <T:Display> ToString for T{
    // --snip
} */

#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

impl Summary for NewsArticle {
    /*
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
    */
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

#[allow(dead_code)]
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

/* pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
} */

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

#[allow(unused_variables)]
pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    // ...
}

#[allow(unused_variables)]
pub fn notify_2(item1: &(impl Summary + Display), item2: &impl Summary) {
    // ...
}

#[allow(unused_variables)]
pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    // ...
}

#[allow(unused_variables)]
pub fn notify_3<T: Summary + Display>(item1: &T, item2: &T) {
    // ...
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    //...
    10
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn some_function_improved<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    //...
    10
}

fn main() {
    let tweet = Tweet {
        username: String::from("@john_doe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The Sky is not actually falling."),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);
    notify(&tweet);
}
