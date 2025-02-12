mod inventory;
mod orders;

fn main() {
    println!(
        "Our managers are {} and {} and we have {} sq feet of floor space",
        inventory::MANAGER,
        orders::MANAGER,
        inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();

    let favourite_category = inventory::products::ProductCategory::Hammer;
    println!("My favourite category is {favourite_category:?}");

    let tall_ladder = inventory::products::Item {
        name: "Ladder-o-matic 2000".to_string(),
        category: favourite_category,
        quantity: 100,
    };

    println!("{tall_ladder:#?}");
}
