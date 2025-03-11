use std::collections::HashSet;

use common::separator;

pub fn run() {
    separator("Map Adapter");
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let my_iterator = numbers.iter();
    let squares = my_iterator.map(|number: &i32| number.pow(2));
    // println!("{:?}", my_iterator); // moved
    println!("{:?}", squares);
    // println!("{:?}", numbers);

    for number in squares {
        println!("Square: {number}");
    }

    for number in numbers.into_iter().map(|number| number.pow(2)) {
        println!("Square: {number}");
    }

    // println!("{:?}", numbers);

    let numbers = vec![4, 8, 4, 15, 16, 23, 42];
    let squares: Vec<_> = numbers.iter().map(|number: &i32| number.pow(2)).collect();
    let squares_hashset = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<HashSet<_>>();

    println!("{squares:?}");
    println!("{squares_hashset:?}");

    let names = [
        String::from("Jimmy"),
        String::from("Cleveland"),
        String::from("Boris"),
    ];

    let name_lengths: Vec<_> = names
        .iter()
        .map(|name| name.to_lowercase())
        .map(|name| name.replace("i", "@@"))
        .map(|name| name.len())
        .collect();

    println!("{name_lengths:?}");

    separator("Filter and Find Adapters");
}
