fn main() {
    let time = 2025;
    let year = time;

    println!("Time is {time}. Year is {year}.");

    let food = "pasta"; // not on stack or heap, but embedded directly in the binary
    let text = String::new(); // this is on heap, its mutable
    let candy = String::from("KitKat");

    let mut name = String::from("Shahrukh");
    name.push_str(" Tramboo");

    println!("{name}");

    let person = String::from("Borris"); // does not implement copy properties
    let genius = person;

    // println!("{person}"); // borrow of moved value: `person`
    drop(genius);
    // println!("{genius}");

    let person = String::from("Borris");
    let mut genius = person.clone();

    println!("person is {person}");

    print_my_string(&mut genius);

    println!("genius is {genius}");

    let handsome = &person;

    println!("handsome is {handsome}"); // display trait for reference automatically dereferences it

    println!("dereference handomse is {}", *handsome);

    /*
       &str ("ref str") - A reference to the text in the memory that has loaded the binary file
    */

    let cake = bake_cake();
    println!("I have a {cake} cake");
}

fn print_my_string(reference: &mut String) {
    // let value = *reference; // cannot move from reference
    println!("through function => {reference}");
    reference.push_str(" and great");
}

fn bake_cake() -> String {
    // let cake = String::from("Chocolate");
    // return cake;
    String::from("Chocolate")
}
