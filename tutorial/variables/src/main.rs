#![allow(unused_variables)]

const TAX_RATE: f64 = 7.25;

type Meters = i32;

#[allow(unused_variables)]
fn main() {
    let apples = 50;
    let oranges = 14 + 6;
    let fruits = apples + oranges;

    let _mile_race_length: Meters = 1600;

    #[allow(unused_variables)]
    let two_miles_race_length: Meters = 3200;

    let grams_of_protein = "100.345";
    let grams_of_protein = 100.345;
    let grams_of_protein = 100;

    println!("My garden has {apples} apples and {oranges} oranges.");
    println!("Tax rate is {TAX_RATE}")
}
