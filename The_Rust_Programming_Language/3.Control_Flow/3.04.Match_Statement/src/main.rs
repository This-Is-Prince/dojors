fn main() {
    let country_code = -1000;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid",
    };
    println!("{} country code is {}", country, country_code);
}
