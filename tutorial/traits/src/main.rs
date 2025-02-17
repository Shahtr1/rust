mod ops;
mod utils;

use ops::MyAdd;
use std::ops::Add;

use common::separator;
use utils::{
    book_for_one_night, book_for_one_night_gen, choose_best_place_to_stay, mix_and_match,
    mix_and_match_gen, mix_and_match_gen_where,
};
use utils::{Accommodation, AirBnB, Description, Hotel};

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

    separator("Multiple Trait Bounds");
    mix_and_match(&mut hotel2, &mut airbnb, "Richy Rich");

    println!("Booked hotel: {:#?}", hotel2);
    println!("Booked AirBnB: {:#?}", airbnb);

    mix_and_match_gen(&mut hotel2, &mut airbnb, "Poory Poor");

    println!("Booked hotel: {:#?}", hotel2);
    println!("Booked AirBnB: {:#?}", airbnb);
    println!("Booked AirBnB: {:#?}", airbnb);

    separator("where Clauses");
    mix_and_match_gen_where(&mut hotel2, &mut airbnb, "Greatly Great");

    println!("Booked hotel: {:#?}", hotel2);
    println!("Booked AirBnB: {:#?}", airbnb);

    separator("Function return Values");
    let mut continental = choose_best_place_to_stay();

    mix_and_match(&mut continental, &mut airbnb, "Crazy Guy");

    println!("Booked Continental: {:#?}", continental);

    // Trait Bounds
    separator("Trait Bounds");

    let hotel_1 = Hotel::new(String::from("The Luxe"));
    println!("{}", hotel_1.summarize());
    let hotel_2 = Hotel::new(String::from("The Golden"));
    println!("{}", hotel_2.summarize());
    let hotel_3 = Hotel::new(vec!["The Sweet", "The Sour"]);
    // println!("{}", hotel_3.summarize()); // won't work

    // Trait Objects
    separator("Trait Objects");
    // Dynamic Dispatches only work with references
    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb];
    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());

    let mut stays: Vec<&mut dyn Accommodation> = vec![&mut hotel, &mut airbnb];
    stays[0].book("Ronaldo", 7);
    stays[1].book("Messi", 10);

    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);

    // Trait Scope
    separator("Trait Scope");

    let a = 5;
    let b = 10;
    let sum = a.add(b);
    println!("{sum}");

    // My Add Trait
    separator("My Add Trait");
    let a = 5;
    let b = 10;
    let sum = a.my_add(b);
    println!("{sum}");
}
