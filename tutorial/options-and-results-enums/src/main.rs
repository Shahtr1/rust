use from_scratch::MyOption;

mod from_scratch;
mod project;

fn operation(great_success: bool) -> Result<String, String> {
    if great_success {
        Ok("Success".to_string())
    } else {
        Err("Error".to_string())
    }
}

fn operation_slices(great_success: bool) -> Result<&'static str, &'static str> {
    if great_success {
        Ok("Success")
    } else {
        Err("Error")
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Denominator cant be 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Some(instrument) => println!("Playing the {instrument}"),
        None => println!("Singing with my voice"),
    }
}

fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Some(true)
    } else if item_is_in_system {
        Some(false)
    } else {
        None
    }
}

fn main() {
    let a = Option::Some(5);
    let b = Option::Some("hello");
    let c = Option::Some(true);

    let a: Option<i8> = Option::Some(5);
    let a = Option::<i16>::Some(5);

    let d: Option<&str> = Option::None;

    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass = musical_instruments.get(2);
    println!("{:?}", bass);
    let valid_instrument = bass.unwrap(); // not the safest
    println!("{valid_instrument}");
    let invalid_instrument = musical_instruments.get(100);
    println!("{:?}", invalid_instrument);
    // invalid_instrument.expect("Unable to retrieve musical instrument");

    // Both unwrap and expect are optimistic, so thats a problem

    let base = play(bass);
    let invalid = play(invalid_instrument);

    let availability = is_item_in_stock(true, false);
    println!("{availability:?}");

    match availability {
        Option::Some(true) => println!("Yes, the item is available"),
        Option::Some(false) => println!("Yes, item is in system, but not available"),
        Option::None => println!("Item doesn't exist in our system"),
    }

    let tabla = &"tabla".to_string();

    let default = invalid_instrument.unwrap_or(tabla);
    println!("{default:?}");
    let bass = bass.unwrap_or(tabla);
    println!("{bass:?}");

    let some_option = MyOption::Some(100);
    println!("{}", some_option.unwrap());

    // let some_option = MyOption::None;
    // println!("{}", some_option.unwrap());

    let ok: Result<i32, &str> = Ok(5); // or Result::Ok(5);
    let disaster: Result<i32, &str> = Err("Something went wrong");
    println!("{:#?}", ok);
    println!("{:#?}", disaster);

    let text = "50";
    let text_as_number = text.parse::<i32>();

    println!("{:?}", text_as_number);

    let text = "abcd";
    let text_as_number = text.parse::<i32>();

    println!("{:?}", text_as_number);

    let result = divide(10.0, 2.0);

    match &result {
        Ok(calculation) => println!("Result: {}", calculation),
        Err(message) => println!("Result: {}", message),
    }

    println!("{}", result.as_ref().unwrap()); // same for expect and unwrap_or

    println!("{}", result.is_ok());
    println!("{}", result.is_err());

    let my_result = operation(true);

    let content = match &my_result {
        // we need reference here, as String dont not implements copy trait
        Ok(message) => message,
        Err(error) => error,
    };

    println!("{}", my_result.unwrap());

    let my_result = operation_slices(true);

    let content = match my_result {
        // we dont need reference here, as &str implements copy trait
        Ok(message) => message,
        Err(error) => error,
    };

    println!("{}", my_result.unwrap());

    let mut sauces = vec!["Mayonaise", "Ketcup", "Ranch"];

    if let Some(sauce) = sauces.pop() {
        println!("The next sauce id {sauce}")
    }

    if let Some(sauce) = sauces.pop() {
        println!("The next sauce id {sauce}")
    }

    if let Some(sauce) = sauces.pop() {
        println!("The next sauce id {sauce}")
    }

    if let Some(sauce) = sauces.pop() {
        // will not execute
        println!("The next sauce id {sauce}")
    }

    // we can do the same with while let
    while let Some(sauce) = sauces.pop() {
        println!("The next sauce id {sauce}")
    }

    project::run();
}
