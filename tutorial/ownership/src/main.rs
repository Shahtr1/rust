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
    let genius = person.clone();

    println!("person is {person}");
    println!("genius is {genius}");

    let handsome = &person;
    println!("handsome is {handsome}"); // display trait for reference automatically dereferences it

    println!("dereference handomse is {}", *handsome);

    /*
       &str ("ref str") - A reference to the text in the memory that has loaded the binary file
    */
}
