mod project;
fn main() {
    let action_hero_literal = "Arnold Schwarzenegger";

    let action_hero = String::from(action_hero_literal);
    let first_name: &str = &action_hero[0..6]; // byte range here, its storing address here of first 6
    println!("{first_name}");
    let last_name = &action_hero[7..21];
    println!("{last_name}");

    let first_name = {
        let action_hero_literal = "Arnold Schwarzenegger";
        &action_hero_literal[0..6]
    };

    println!("{first_name}");
    println!("{first_name:p}"); // memory address of string data
    println!("{:p}", &action_hero_literal); // memory address of action_hero_literal

    let food = "🍕";
    println!("{}", food.len()); // its always number of bytes, representing a single character
    let pizza_slice = &food[0..4]; // not have a valid character boundary at 3
    println!("{pizza_slice}");

    // String slices are created at runtim, so it doesnt matter we are getting them from heap or literal

    let first_name: &str = &action_hero[..6]; // same as 0..6
    println!("{first_name}");
    let last_name = &action_hero[7..]; // same as 7..21
    println!("{last_name}");

    let full_name = &action_hero[..]; // from beginning to end
    println!("{full_name}");

    do_hero_stuff(&action_hero); // this is deref coercion, rust converts &String -> &str
    let another_action_hero = "Sylvester Stallone";
    do_hero_stuff(another_action_hero);

    let values = [4, 8, 15, 16, 23, 42];

    let my_slice = &values[..3];

    dbg!(my_slice);

    let regular_reference = &values;

    print_length(regular_reference);
    print_length(my_slice);

    let mut my_array = [10, 15, 20, 25, 30];
    let my_slice = &mut my_array[2..4];
    my_slice[0] = 5;
    dbg!(my_slice);
    dbg!(my_array);

    project::run();
}

fn print_length(reference: &[i32]) {
    println!("{}", reference.len());
}

fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}
