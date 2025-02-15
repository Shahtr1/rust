mod project;

// use std::process;

use std::{
    fs,
    io::{stdin, Error},
    process,
};

fn main() {
    // None.unwrap(); // panic
    // panic!("Soemthing went wrong!");

    // process::exit(1); // something went wrong
    // process::exit(0); // everything went good

    // println!("Some status message");
    // eprintln!("Some error message");

    let file_result = read_file();

    match file_result {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprint!("Something went wrong. The error was {error:#?}");
            process::exit(1)
        }
    }

    let mut animals = vec!["Giraffe", "Monkey", "Zebra"];

    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));

    project::run();
}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())
}

fn read_file() -> Result<String, Error> {
    println!("Please enter the name of a file you'd like to read:");
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    fs::read_to_string(input.trim())

    // comment below line because above line does it cleanly
    // let mut contents = String::new();
    // File::open(input.trim())?.read_to_string(&mut contents)?;

    // return Ok(contents);
}
