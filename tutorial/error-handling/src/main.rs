// use std::process;

use std::{fs::File, io::stdin, process};

fn main() {
    // None.unwrap(); // panic
    // panic!("Soemthing went wrong!");

    // process::exit(1); // something went wrong
    // process::exit(0); // everything went good

    println!("Some status message");
    eprintln!("Some error message");

    println!("Please enter the name of a file you'd like to read:");
    let mut input = String::new();

    let user_request_file = stdin().read_line(&mut input);

    if let Err(error) = user_request_file {
        eprint!("Something went wrong. The error was {error:#?}");
        process::exit(1)
    }

    let file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprint!("Something went wrong. The error was {error}");
            process::exit(1)
        }
    };
    println!("{file:#?}");

    let file = File::open("nonsense.txt");
    println!("{file:#?}");
}
