fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}");

    let evaluation = true;
    match evaluation {
        true => {
            println!("The value is true");
        }
        false => {
            println!("The value is false");
        }
    }

    let value = match evaluation {
        true => 20,
        false => 40,
    };

    let season = "shahrukh";

    match season {
        "summer" => println!("School's out"),
        "winter" => println!("Brr, so cold"),
        "autumn" | "spring" => println!("autumn or spring"),
        value if (value == "shahrukh") => println!("You printed {value}"),
        // _ => println!("Lots of rain"), // default value
        // or
        _ => unreachable!(), // to satisfy the rust compiler
    }
}

fn main() {
    even_or_odd(3);
}
