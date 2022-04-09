const NUMBER_OF_MONTHS: u32 = 12;
static SEASONS: [&str; 4] = ["Spring", "Summer", "Fail", "Winter"];

fn main() {
    println!("Const And Static");
    println!("{}", NUMBER_OF_MONTHS);
    println!("{:#?}", SEASONS);
}
