mod utils;

use common::separator;
use utils::{Accommodation, Description};

fn main() {
    let mut hotel = utils::Hotel::new("The Luxe");
    println!("{}", hotel.get_description());
    hotel.book("Peter", 4);
    println!("{hotel:#?}");

    let mut airbnb = utils::AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Raj", 9);
    println!("{airbnb:#?}");

    println!("{}", hotel.summarize());

    let mut hotel2 = utils::Hotel::new("Grand Hotel");
    println!("Booking for one night");
    utils::book_for_one_night(&mut hotel2, "Amanda");
    println!("Booked hotel: {:#?}", hotel2);
    println!("Booking for one night");
    utils::book_for_one_night_gen(&mut hotel2, "Gamanda");
    println!("Booked hotel: {:#?}", hotel2);

    utils::mix_and_match(&mut hotel2, &mut airbnb, "Richy Rich");

    println!("Booked hotel: {:#?}", hotel2);
    println!("Booked AirBnB: {:#?}", airbnb);

    utils::mix_and_match_gen(&mut hotel2, &mut airbnb, "Poory Poor");

    println!("Booked hotel: {:#?}", hotel2);
    println!("Booked AirBnB: {:#?}", airbnb);
    println!("Booked AirBnB: {:#?}", airbnb);

    utils::mix_and_match_gen_where(&mut hotel2, &mut airbnb, "Greatly Great");

    println!("Booked hotel: {:#?}", hotel2);
    println!("Booked AirBnB: {:#?}", airbnb);

    let mut continental = utils::choose_best_place_to_stay();

    utils::mix_and_match(&mut continental, &mut airbnb, "Crazy Guy");

    println!("Booked Continental: {:#?}", continental);

    // Trait Bounds
    separator("Trait Bounds");

    let hotel_1 = utils::Hotel::new(String::from("The Luxe"));
    println!("{}", hotel_1.summarize());
    let hotel_2 = utils::Hotel::new(String::from("The Golden"));
    println!("{}", hotel_2.summarize());
    let hotel_3 = utils::Hotel::new(vec!["The Sweet", "The Sour"]);
    // println!("{}", hotel_3.summarize()); // won't work
}
