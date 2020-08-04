struct Country {
    population: u32,
    capital: String,
    leader_name: String
}
enum ThingsInTheSky {
    Sun,
    Stars,
}

// With this function we can use an i32 to create ThingsInTheSky.
fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun, // Between 6 and 18 hours we can see the sun
        _ => ThingsInTheSky::Stars, // Otherwise, we can see stars
    }
}

// With this function we can match against the two choices in ThingsInTheSky.
fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!")
    }
}

enum Number {
    U32(u32),
    I32(i32),
}

impl Number {
    fn new(number: i32) -> Number { // input number is i32
        match number.is_positive() {
            true => Number::U32(number as u32), // change it to u32 if it's positive
            false => Number::I32(number), // otherwise just give the number because it's already i32
        }
    }
}

fn main() {
    let population = 500_000;
    let capital = String::from("Elist");
    let leader_name = String::from("Batu Khasikov");

    let kalmykia = Country {
        population,
        capital,
        leader_name,
    };
    let time = 8; // it's 8 o'clock
    let skystate = create_skystate(time); // create_skystate returns a ThingsInTheSky
    check_skystate(&skystate); // Give it a reference so it can read the variable skystate

    let my_vec = vec![Number::new(-800), Number::new(8)];

    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {}", number),
            Number::I32(number) => println!("It's a i32 with the value {}", number),
        }
    }
    struct Item {
        number: u8,
    }
    let item = Item {
        number: 8,
    };

    let reference_item = &item;

    println!("{}", reference_item.number == 8); // we don't need to write *reference_item.number

    struct Person { // make a simple struct for a person
    name: String,
        real_name: String,
        height: u8,
        happiness: bool
    }
    let papa_doc = Person { // create variable papa_doc
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false
    };

    let Person { // destructure papa_doc
        name: a,
        real_name: b,
        height: c,
        happiness: d
    } = papa_doc;

    println!("Our four values are: {}, {}, {}, and {}.", d, c, b, a);

    struct City {
        name: String,
        name_before: String,
        population: u32,
        date_founded: u32,
    }

    impl City {
        fn new(name: String, name_before: String, population: u32, date_founded: u32) -> Self {
            Self {
                name,
                name_before,
                population,
                date_founded,
            }
        }
    }
    fn process_city_values(city: &City) {
        let City {
            name,
            name_before,
            population,
            date_founded,
        } = city;
        // now we have the values to use separately
        let two_names = vec![name, name_before];
        println!("{:?}", two_names);
    }
    let tallinn = City::new("Tallinn".to_string(), "Reval".to_string(), 426_538, 1219);
    process_city_values(&tallinn);

    let mut counter = 5;
    let my_number = loop {
        counter +=1;
        if counter % 53 == 3 {
            break counter;
        }
    };
    println!("{}", my_number);

    fn match_colours(rbg: (i32, i32, i32)) {
        let new_vec = vec![(rbg.0, "red"), (rbg.1, "blue"), (rbg.2, "green")]; // Put the colours in a vec. Inside are tuples with the colour names
        let mut all_have_at_least_10 = true; // Start with true. We will set it to false if one colour is less than 10
        for item in new_vec {
            if item.0 < 10 {
                all_have_at_least_10 = false; // Now it's false
                println!("item 0 is  {}.", item.0);
                println!("Not much {}.", item.1) // And we print the colour name.
            }
        }
        if all_have_at_least_10 { // Check if it's still true, and print if true
            println!("Each colour has at least 10.")
        }
    }
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
}