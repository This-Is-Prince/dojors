/*
=============
Basic Enum
=============
*/
#[allow(dead_code)]
// create the enum with two choices
enum ThingsInSky1 {
    Sun,
    Stars,
}

// With This function we can use an i32 to create ThingsInTheSky1.
fn create_sky_state1(time: i32) -> ThingsInSky1 {
    match time {
        6..=18 => ThingsInSky1::Sun, // Between 6 and 18 hours we see the sun
        _ => ThingsInSky1::Stars,    // Otherwise, we can see stars
    }
}

// With the function we can match against the two choices in ThingsInTheSky1.
fn check_sky_state1(state: &ThingsInSky1) {
    match state {
        ThingsInSky1::Sun => println!("I can see the sun!"),
        ThingsInSky1::Stars => println!("I can see the stars!"),
    }
}

/*
=============
Giving Data to each variant
=============
*/
#[allow(dead_code)]
enum ThingsInSky2 {
    Sun(String), // Now each variant has a string
    Stars(String),
}

fn create_sky_state2(time: i32) -> ThingsInSky2 {
    match time {
        6..=18 => ThingsInSky2::Sun(String::from("I can see the sun!")), // Write the strings here
        _ => ThingsInSky2::Stars(String::from("I can see the stars!")),
    }
}

fn check_sky_state2(state: &ThingsInSky2) {
    match state {
        ThingsInSky2::Sun(description) => println!("{}", description), // Give the string the name description so we can use it
        ThingsInSky2::Stars(n) => println!("{}", n), // Or you can name it n. Or anything else - it doesn't matter
    }
}

/*
=============
Import EveryThing
=============
*/
#[allow(dead_code)]
enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

fn match_mood1(mood: &Mood) -> i32 {
    let happiness_level = match mood {
        Mood::Happy => 10, // Here we type Mood:: every time
        Mood::Sleepy => 6,
        Mood::NotBad => 7,
        Mood::Angry => 2,
    };
    happiness_level
}

fn match_mood2(mood: &Mood) -> i32 {
    use Mood::*; // We imported everything in Mood. Now we can just write Happy, Sleepy, NotBad, Angry etc.
    let happiness_level = match mood {
        Happy => 10, // We don't have to write Mood:: anymore
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    };
    happiness_level
}

/*
=============
Parts of an enum can also be turned into an integer.
=============
*/
#[allow(dead_code)]
enum Season {
    Spring, // If this was Spring(String) or something it wouldn't work
    Summer,
    Autumn,
    Winter,
}

/*
=============
Giving Different number to each Star
=============
*/
#[allow(dead_code)]
enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar, // Think about this one. What number will it have?
}

/*
=============
Enums to use multiple types
=============
*/
#[allow(dead_code)]
enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32), // change it to u32 if it's positive
        false => Number::I32(input), // Otherwise just give the number because it's already i32
    };
    number
}

fn main() {
    println!("Enums");

    let time = 8; // it's 8 o'clock
    let sky_state1 = create_sky_state1(time); // create_sky_state1 return a ThingsInTheSky1
    check_sky_state1(&sky_state1); // Give it a reference so it can read the variable sky_state1

    let time = 8; // it's 8 o'clock
    let sky_state2 = create_sky_state2(time); // create_sky_state2 return a ThingsInTheSky2
    check_sky_state2(&sky_state2); // Give it a reference so it can read the variable sky_state2

    let my_mood = Mood::NotBad;
    let happiness_level = match_mood1(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);

    let my_mood = Mood::Angry;
    let happiness_level = match_mood2(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);

    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("{}", season as i32);
    }

    use Star::*;
    let star_vec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant];
    for star in star_vec {
        match star as i32 {
            size if size <= 80 => println!("Not the biggest star."), // Remember: size doesn't mean anything. It's just a name we chose so we can print it
            size if size >= 80 => println!("This is a good-sized star."),
            _ => println!("That star is pretty big!"),
        }
    }
    println!("What about DeadStar? It's the number {}.", DeadStar as u32);

    let my_vec = vec![get_number(-800), get_number(8)];
    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {}", number),
            Number::I32(number) => println!("It's a i32 with the value {}", number),
        }
    }
}
