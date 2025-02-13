use std::collections::HashMap;

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

    let mut coffee_pairings = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(drink, milk);
    println!("{:?}", coffee_pairings);
    // println!("{:?}", milk); // error

    let mut coffee_pairings = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    println!("{:?}", milk); // not an error
}
