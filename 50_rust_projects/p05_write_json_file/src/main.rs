use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    println!("JSON WRITE");

    let article = Article {
        article: String::from("How to write json from rust struct"),
        author: String::from("Prince"),
        paragraph: vec![
            Paragraph {
                name: String::from("First Paragraph"),
            },
            Paragraph {
                name: String::from("Second Paragraph"),
            },
            Paragraph {
                name: String::from("Third Paragraph"),
            },
        ],
    };

    let raw_json = serde_json::to_string(&article).unwrap();
    println!("The JSON is: {}", raw_json)
}
