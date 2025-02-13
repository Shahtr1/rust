use std::collections::HashMap;

pub fn run() {
    let mut sauces_to_meals = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);

    let removed_option = sauces_to_meals.remove("Mayonnaise");

    let removed_vector = removed_option.expect("No Key found");

    println!("Removed vector: {:#?}", removed_vector);

    let mustard = sauces_to_meals.get("Mustard").expect("No Key found");

    println!("Removed vector: {:#?}", mustard);

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("{:#?}", sauces_to_meals);
}
