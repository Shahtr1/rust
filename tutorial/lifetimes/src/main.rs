mod project;

use std::char;

struct DentistAppointment {
    doctor: String,
}

impl DentistAppointment {
    // Elision rule 2, lifetime of self is connected to lifetime of returned value
    fn book(&self, check_in_time: &str, check_out_time: &str) -> &str {
        &self.doctor
    }

    fn book_2<'a, 'b, 'c>(&'a self, check_in_time: &'b str, check_out_time: &'c str) -> &'b str {
        check_in_time
    }
}

#[derive(Debug)]
struct TrainSystem<'a> {
    name: &'a str,
}

struct TravelPlan<'a> {
    from: &'a str,
    to: &'a str,
}

fn create() -> i32 {
    let age = 100;
    // &age // creates a dangling reference
    age
}

fn create_number_reference(number: i32) -> i32 {
    // &number // creates a dongling reference
    number
}

fn capitalize(char_array: &mut [char]) {
    for char in char_array.iter_mut() {
        *char = char.to_uppercase().next().unwrap_or(*char);
    }

    println!("{:?}", char_array);
}

fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    // let selected_items = &items[..2];
    // print!("{selected_items:?}");
    &items[..2]
}

fn choose_favourite<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    first
}

fn choose_bad<'a, 'b>(first: &'a str, second: &'a str) -> &'a str {
    if true {
        first
    } else {
        second
    }
}

/*
fn figure_out_ending_point<'a>(from: &'a str) -> &'a str {
    let to = String::from("Paris");

    let travel_plan = TravelPlan {
        from: from,
        to: &to,
    };

    travel_plan.from
}
     */

fn say_hello() -> &'static str {
    "Hello"
}

fn main() {
    let from = String::from("London");

    /*
    let plan = {
        let to = String::from("Paris");
        let travel_plan = TravelPlan {
            from: &from,
            to: &to,
        };

        travel_plan.from
        // travel plan wil be valid after this, because `to` is dropped here
    };
     */

    let first = String::from("First String"); // Long-lived
    let fav;

    {
        let second = String::from("Second String"); // Short-lived (lives only in this block)
        fav = choose_favourite(&first, &second); // ❌ ERROR
    } // `second` is dropped here!

    println!("{}", fav);

    let dog = String::from("Watson"); // referrant or lender
    let my_pet = &dog;
    println!("{my_pet}");

    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..];
    capitalize(slice);

    data.push('d');
    data.push('e');
    data.push('f');

    // println!("{:?}", slice); // will cause error above

    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let favourite_cities = &cities[0..2];

    // drop(cities);

    println!("{favourite_cities:?}");

    /*
    // This also wont work
    let some_cities = {
        let cities = vec![
            String::from("London"),
            String::from("New York"),
            String::from("Barcelona"),
        ];

        &cities[0..2] // returns dangling reference
    };
    */

    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];

    let two_cities = select_first_two_elements(&cities);
    // drop(cities); // error
    println!("{two_cities:?}");

    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{two_coffees:?}")
    }

    let train_system = TrainSystem {
        name: &String::from("London Underground"),
    };

    println!("{:?}", train_system);

    project::run();
}
