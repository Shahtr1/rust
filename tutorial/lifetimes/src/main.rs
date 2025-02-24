use std::char;

fn create() -> i32 {
    let age = 100;
    // &age // creates a dangling reference
    age
}

fn create_number_reference(number: i32) -> i32 {
    // &number // creates a dongling reference
    number
}

fn capitalize(char_array: &mut [char]) {
    for char in char_array.iter_mut() {
        *char = char.to_uppercase().next().unwrap_or(*char);
    }

    println!("{:?}", char_array);
}

fn main() {
    let dog = String::from("Watson"); // referrant or lender
    let my_pet = &dog;
    println!("{my_pet}");

    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..];
    capitalize(slice);

    data.push('d');
    data.push('e');
    data.push('f');

    // println!("{:?}", slice); // will cause error above

    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let favourite_cities = &cities[0..2];

    // drop(cities);

    println!("{favourite_cities:?}");

    /*
    // This also wont work
    let some_cities = {
        let cities = vec![
            String::from("London"),
            String::from("New York"),
            String::from("Barcelona"),
        ];

        &cities[0..2] // returns dangling reference
    };
    */
}
