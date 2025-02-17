use common::separator;

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

#[derive(Debug)]

struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1);
}

fn book_for_one_night_gen<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

fn mix_and_match(
    first: &mut (impl Accommodation + Description),
    second: &mut impl Accommodation,
    guest: &str,
) {
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 1);
}

fn mix_and_match_gen<T: Accommodation + Description, U: Accommodation>(
    first: &mut T,
    second: &mut U,
    guest: &str,
) {
    first.book(guest, 1);
    first.get_description();

    second.book(guest, 1);
}

fn mix_and_match_gen_where<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    first.get_description();

    second.book(guest, 1);
}

fn choose_best_place_to_stay() -> impl Accommodation + Description + Debug {
    let likes_luxury = true;
    if likes_luxury {
        Hotel::new("Lux Continental")
    } else {
        Hotel::new("Continental")
    }
}

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.get_description());
    hotel.book("Peter", 4);
    println!("{hotel:#?}");

    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Raj", 9);
    println!("{airbnb:#?}");

    println!("{}", hotel.summarize());

    let mut hotel2 = Hotel::new("Grand Hotel");
    println!("Booking for one night");
    book_for_one_night(&mut hotel2, "Amanda");
    println!("Booked hotel: {:#?}", hotel2);
    println!("Booking for one night");
    book_for_one_night_gen(&mut hotel2, "Gamanda");
    println!("Booked hotel: {:#?}", hotel2);

    mix_and_match(&mut hotel2, &mut airbnb, "Richy Rich");

    println!("Booked hotel: {:#?}", hotel2);
    println!("Booked AirBnB: {:#?}", airbnb);

    mix_and_match_gen(&mut hotel2, &mut airbnb, "Poory Poor");

    println!("Booked hotel: {:#?}", hotel2);
    println!("Booked AirBnB: {:#?}", airbnb);
    println!("Booked AirBnB: {:#?}", airbnb);

    mix_and_match_gen_where(&mut hotel2, &mut airbnb, "Greatly Great");

    println!("Booked hotel: {:#?}", hotel2);
    println!("Booked AirBnB: {:#?}", airbnb);

    let mut continental = choose_best_place_to_stay();

    mix_and_match(&mut continental, &mut airbnb, "Crazy Guy");

    println!("Booked Continental: {:#?}", continental);

    separator("Trait Bounds");
}
