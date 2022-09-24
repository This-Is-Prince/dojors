const BRACES: [u8; 3] = [b'{', b'(', b'['];
fn main() {
    println!("Hello, world!");

    println!("{}",brackets_are_balanced("[{((string)}]"))
}

fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<u8> = Vec::new();
    string.as_bytes().iter().all(|b| match b {
        b'}' => stack.pop() == Some(b'{'),
        b')' => stack.pop() == Some(b'('),
        b']' => stack.pop() == Some(b'['),
        _ => {
            if BRACES.contains(b) {
                stack.push(*b);
            }
            true
        }
    }) && stack.len() == 0
}
