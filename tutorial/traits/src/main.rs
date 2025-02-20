use std::str::FromStr;
use std::{fs, ops::Add};

use common::separator;
use traits::std_traits_impl::{Appointment, Duration};
use traits::{
    common_traits::{convert_traits, defaultValue},
    finance::{Bonus, Income, Investment, QualityTime, Taxable},
    lodging::{Accommodation, AirBnB, Description, Hotel},
    ops::MyAdd,
    std_traits_impl::{Apple, AppleType},
    utils::{
        book_for_one_night, book_for_one_night_gen, choose_best_place_to_stay, mix_and_match,
        mix_and_match_gen, mix_and_match_gen_where,
    },
};

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

    separator("Conversions");
    let numeric_count = u64::from_str("5");
    println!("{}", numeric_count.unwrap());
    let new_type = convert_traits::<u32, u64>(14);
    let new_type = convert_traits::<i32, i64>(-14);
    let new_type = convert_traits::<&str, String>("hi");
    let string_vector = convert_traits::<&str, Vec<u8>>("Hello");
    println!("Hello to u8 vector: {:?}", string_vector);

    separator("Default");
    #[derive(Debug)]
    struct MyStruct {
        name: &'static str,
    }

    impl Default for MyStruct {
        fn default() -> Self {
            Self { name: "Shahrukh" }
        }
    }

    println!("{:#?}", defaultValue(MyStruct { name: "Jerry" }));

    separator("Associated Constants and Generics");

    let mut income = Income { amount: 50000.50 };
    println!("Total tax owed: {:.2}", income.tax_bill());

    let mut bonus = Bonus { value: 50000.50 };
    println!("Total bonus: {:.2}", bonus.tax_bill());

    income.double_amount();
    println!("Total income doubled: {:.2}", income.tax_bill());

    bonus.double_amount();
    println!("Total bonus doubled: {:.2}", bonus.tax_bill());

    let weekend = QualityTime { minutes: 120 };
    println!("Relaxation time: {:.2} minutes", weekend.amount());

    separator("Implementing Display Trait");

    let lunch_snack = Apple {
        kind: AppleType::GrannySmiths,
        price: 1.04,
    };

    println!("{}", lunch_snack);
    println!("{:?}", lunch_snack);

    let lunch_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 2.50,
    };

    println!("{}", lunch_snack);
    println!("{:#?}", lunch_snack);

    separator("Implementing Clone Trait");
    let morning_appt = Appointment::new("Dr. Andrews", "9:00AM", "10:00AM");
    let replacement_app = morning_appt.clone();
    println!(
        "{} is seeing the patient from {} to {}",
        replacement_app.doctor, replacement_app.start_time, replacement_app.end_time
    );

    separator("Implementing Copy Trait");
    let one_hour = Duration::new(1, 0, 0);
    let another_hour = one_hour;
    println!("{:?}", one_hour);

    separator("Implementing Partial Equality Trait");
}
