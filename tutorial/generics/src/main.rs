mod project;

#[derive(Debug)]
enum Cheeseteak<T> {
    Plain,
    Toppin(T),
}

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl<T> TreasureChest<T> {
    fn new(captain: String, treasure: T) -> Self {
        TreasureChest { captain, treasure }
    }

    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

fn identity<T>(value: T) -> T {
    value
}

fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn main() {
    let value = identity(String::from("Shahrukh"));
    let value = identity::<i64>(5);

    let my_tuple = make_tuple("Hello", 7);
    let my_tuple = make_tuple(5, 6);

    let chest = TreasureChest::new(String::from("Jack Sparrow"), "Gold");
    let chest = TreasureChest::new(String::from("Jack Sparrow"), 199);

    let mut gold_chest = TreasureChest {
        captain: String::from("Jack Sparrow"),
        treasure: String::from("  Blackpearl  "),
    };

    gold_chest.clean_treasure();
    println!("In capital: {}", gold_chest.capital_captain());

    println!("{gold_chest:#?}");

    let silver_chest = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: ["Gold", "Silver", "Platinum"],
    };

    let amount = silver_chest.amount_of_treasure();
    println!("amount is {amount}");

    println!("In capital: {}", silver_chest.capital_captain());

    let other_chest = TreasureChest {
        captain: String::from("WaterSail"),
        treasure: "Nothing",
    };

    println!("{other_chest:#?}");
    println!("In capital: {}", other_chest.capital_captain());

    let mushroom = Cheeseteak::Toppin("mushroom");
    let onions = Cheeseteak::Toppin("onions".to_string());
    let topping = "bacon".to_string();
    let bacon = Cheeseteak::Toppin(&topping);

    let mut plain: Cheeseteak<String> = Cheeseteak::Plain;

    plain = Cheeseteak::Toppin("sausage".to_string());

    project::run();
}
