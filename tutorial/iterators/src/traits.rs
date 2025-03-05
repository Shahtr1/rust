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
}
