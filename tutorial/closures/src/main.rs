mod custom;

use std::io::stdin;

use common::separator;
use custom::Vault;

fn main() {
    separator("Fn()");
    let multiplier = 5;

    let multiply_by = |value: i32| -> i32 { value * multiplier };

    println!("{}", multiply_by(5));

    let product = |a, b| a * b;
    println!("{}", product(5 as u8, 2));
    // println!("{}", product(1000, 2)); error as inferred type is u8

    let mut numbers = vec![4, 8, 15, 16, 23, 42];
    let print_numbers = || {
        println!("{:?}", numbers);
    };
    print_numbers();
    println!("{:?}", numbers);

    separator("FnMut()");
    let mut add_number = || numbers.push(100);
    add_number();
    add_number();

    println!("{:?}", numbers);

    separator("FnOnce()");
    let number = 13;
    let capture_number = || number;
    let a = capture_number();
    let b = capture_number();
    println!("{a},{b},{number}");

    let first_name = String::from("Alice");
    let capture_string = || first_name;
    // println!("{first_name}"); // no longer the owner
    let owner = capture_string();
    // capture_string(); // cannot execute more than once

    let first_name = String::from("Alice");
    let capture_string_2 = || {
        let person = first_name;
        println!("{person}")
    };
    capture_string_2();
    // capture_string_2(); // can't call again

    separator("Move keyword");
    let first_name = String::from("Alice");
    let last_name = String::from("Wonderland");
    let capture_string_3 = move || {
        print!("{first_name} ");
        println!("{last_name}");
    };

    // println!("{first_name}"); // error: all value sin closure are moved

    capture_string_3();
    capture_string_3();
    capture_string_3();
    println!("All three will be allowed to call, as move occurs but not owned");

    separator("unwrap_or_else");

    let option = Some("Salami");
    let food = option.unwrap_or_else(|| "Pizza");
    println!("{food}");

    let option: Option<&str> = None;
    let food = option.unwrap_or_else(|| "Pizza");
    println!("{food}");

    separator("Custom Fn impl");
    let vault = Vault {
        password: String::from("topsecret"),
        treasure: String::from("Gold"),
    };

    let hack = || {
        let mut user_input = String::new();
        println!("Please provide a password to crack the vault");
        stdin().read_line(&mut user_input);
        user_input = user_input.trim().to_string();
        user_input
    };

    let extraction = vault.unlock(hack);
    println!("{:?}", extraction);
}
