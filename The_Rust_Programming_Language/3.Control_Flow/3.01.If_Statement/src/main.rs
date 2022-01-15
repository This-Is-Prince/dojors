fn if_statement() {
    let temp = 25;
    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10 {
        println!("really cold!");
    } else {
        println!("temperature is OK");
    }

    let day = if temp > -20 {
        "sunny"
    } else if temp < 0 {
        "cold!"
    } else {
        "cloudy"
    };
    println!("today is {}", day);

    println!(
        "is it {}",
        if temp > 20 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    )
}

fn main() {
    if_statement();
}
