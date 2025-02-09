mod project;

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

    let mut car = String::from("Red");
    // let ref1 = &car;
    // a value can have only one single mutable reference at the same time,
    // also you cannot have mutable reference when immutabe reference already exists
    let ref3 = &mut car;
    let ref1 = &car;
    let ref2 = &car;
    println!("{ref1} and {ref2} and {}", &car);

    // immutable reference implements the copy trait
    let coffee = String::from("Mocha");
    let a = &coffee;
    let b = a;

    println!("{} is equal to {}", a, b);

    // mutable references do not implement the copy trait
    let mut coffee = String::from("Mocha");
    let a = &mut coffee;
    let b = a;
    // println!("{} is equal to {}", a, b); // ownership moves to b here

    let registration = [true, false, true];
    let first = registration[0];
    println!("{first} and {registration:?}");

    let languages = [String::from("Rust"), String::from("Java")];
    let first = languages[0].clone(); // not recommended as it creates duplicates in heap
    let second: &String = &languages[0];
    println!("{first} and {second} and {registration:?}");

    project::run();
}

// dangling references
/*
    fn create_city() -> &String {
        let city = String::from("New York");
        &city // this return value represents here a dangling references
    }
*/

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
