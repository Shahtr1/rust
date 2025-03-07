use std::collections::HashMap;

use common::separator;

pub fn iteration() {
    separator("Iterator");
    let vector = vec![4, 8, 15, 16, 23, 42];
    let mut vec_iterator = vector.into_iter(); // transfers ownership

    // println!("{vector:?}"); // error

    let mut hashmap = HashMap::new();
    hashmap.insert("CBS", 2);
    let iterator = hashmap.into_iter();

    println!("vec_iterator {:?}", vec_iterator);
    println!("vec_iterator {:?}", vec_iterator.next());
    println!("vec_iterator {:?}", vec_iterator.next());
    println!("vec_iterator {:?}", vec_iterator.next());
    println!("vec_iterator {:?}", vec_iterator.next());

    println!("vec_iterator {:?}", vec_iterator);

    // iterator returns itself
    for number in vec_iterator {
        println!("{number}")
    }

    // println!("vec_iterator {:?}", vec_iterator.next()); // ownership moved

    separator("iter() method");

    let vector = vec![4, 8, 15, 16, 23, 42];
    // let iterator = vector.iter(); can be shprtened by directly use &vector

    for number in &vector {
        println!("{number}")
    }

    println!("{vector:?}");

    let mut cities = vec![
        "New York".to_string(),
        "London".to_string(),
        "Tokyo".to_string(),
    ];

    for city in &cities {
        println!("{city}")
    }

    println!("{cities:?}");

    separator("IterMut trait");

    let iterator = cities.iter_mut();

    for flavour in iterator {
        flavour.push_str(" Ice Cream");
    }

    println!("{cities:?}");

    for flavour in &mut cities {
        flavour.push_str(" More...");
    }

    println!("{cities:?}");

    separator("Explicitly telling rust to change at address");
    let mut school_grades = [85, 90, 72, 92];

    for grade in &mut school_grades {
        // grade -= 2; // wont work, thinks we are trying to subratc from a reference
        *grade -= 2;
    }

    println!("{school_grades:?}");

    // OWNERSHIP
    // for value in collection
    // for value in collection.into_iter()

    // IMMUTABLE REFERENCES
    // for value in &collection
    // for value in collection.iter()

    // MUTABLE REFERENCES
    // for value in &mut collection
    // for value in collection.iter_mut()

    separator("HashMap");

    let mut todos = HashMap::new();
    todos.insert("Pick up", false);
    todos.insert("Drop down", false);
    todos.insert("Throw right", false);

    for (todo, completion_status) in &mut todos {
        println!("Task: {todo}. Complete: {completion_status}");
        *completion_status = true;
    }

    println!("{todos:?}");

    separator("String or &str");

    let seafood = "Oyster🦪";

    for byte in seafood.bytes() {
        print!("{byte}/");
    }

    println!("{seafood}");

    for charcter in seafood.chars() {
        print!("{charcter}/");
    }

    println!("{seafood}");

    // exhausts the iterator
    println!("{}", seafood.bytes().len());
    println!("{}", seafood.chars().count());
}
