mod project;

use std::collections::{HashMap, HashSet};

fn main() {
    // let mut menu: HashMap<String, f64> = HashMap::new();
    let mut menu = HashMap::new();

    menu.insert("Steak".to_string(), 29.99);
    menu.insert("Tuna".to_string(), 25.99);
    menu.insert("Burger".to_string(), 14.99);

    println!("{menu:?}");

    let data = [("Bobby", 7), ("Grant", 4), ("Ben", 6)];

    let mut years_at_company = HashMap::from(data);
    println!("{:?}", years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("{:?}", ben.unwrap());
    println!("{:?}", years_at_company);

    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    println!("{:?}", coffee_pairings);
    // println!("{:?}", milk); // error

    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    println!("{:?}", milk); // not an error

    coffee_pairings.insert("Flat White", "Almond Milk");
    coffee_pairings.insert("Flat White", "Overwritten Almond Milk");

    // let value = coffee_pairings["Latte"];
    // let value = coffee_pairings.get("Latte").expect("No Key present");
    let mut value: &str = coffee_pairings
        .get("Flat White")
        .copied()
        .expect("No Value present");
    println!("{}", value);

    coffee_pairings
        .entry("Latte")
        .or_insert("Override the overriden Milk");

    println!("{coffee_pairings:?}");

    coffee_pairings
        .entry("Cappuccino")
        .or_insert("Pistacho Milk");

    println!("{coffee_pairings:?}");

    // HashSet => Uses HashMap behind the scenes

    let mut concert_queue = HashSet::new();

    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    concert_queue.insert("Megan");

    println!("{:?}", concert_queue);

    println!("{}", concert_queue.remove("Megan"));
    println!("{}", concert_queue.remove("Franny"));

    println!("{}", concert_queue.contains("Molly"));
    println!("{}", concert_queue.contains("Franny"));

    let molly = concert_queue.get("Molly").copied();
    println!("{:?}", molly);

    let mut movie_queue = HashSet::new();
    movie_queue.insert("Molly");
    movie_queue.insert("Megan");
    movie_queue.insert("Phil");

    println!("{:?}", concert_queue.union(&movie_queue));

    println!("{:?}", movie_queue.difference(&concert_queue));

    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));

    println!("{:?}", concert_queue.is_disjoint(&movie_queue)); // true if no shared value

    println!("{:?}", concert_queue.is_subset(&movie_queue));

    println!("{:?}", movie_queue.is_superset(&concert_queue));

    project::run();
}
