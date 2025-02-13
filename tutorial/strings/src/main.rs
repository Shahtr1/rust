use std::io;

fn main() {
    println!("Hello, world!");

    let pirate = "Bloodhook";

    let sailer = String::from(pirate);

    let first_initial = &sailer[0..1];

    println!("{first_initial}");

    let mut full_name = String::from("Shahrukh");
    let last_name = String::from("Tramboo");

    full_name.push(' ');
    full_name.push_str(&last_name); // &String -> &str

    let first_name = String::from("Shahrukh");

    let full_name = first_name + &last_name;

    // calls first_name.add(&last_name), takes first_name's ownership

    println!("{full_name}");
    // println!("{first_name}"); // error

    let first_name = String::from("Shahrukh");

    let icon = format!("{first_name} {last_name}");
    println!("{icon}");
    println!("{first_name}");
    println!("{last_name}");

    let mut music_genres = "  Rock, Metal, Country, Rap  ";
    println!("{}", music_genres.trim());

    println!("{music_genres}");

    music_genres = music_genres.trim();

    println!("{music_genres}");
    println!("{}", music_genres.replace("a", "@"));

    let genres: Vec<&str> = music_genres.split(", ").collect();

    println!("{:?}", genres);

    let mut name = String::new();

    println!("What is your name?");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to collect input from the user");
    println!("Hello, {name}");
    println!("Hello, {}", name.trim());
}
