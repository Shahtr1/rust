use core::num;
use std::collections::{HashMap, HashSet};

use common::separator;

#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials,
}

#[derive(Debug)]
struct TVChannel {
    name: String,
    channel_type: ChannelType,
}

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

    let numbers = [4, 8, 15, 16, 23, 42];

    let evens: Vec<i32> = numbers
        .into_iter()
        .filter(|number| number % 2 == 0)
        .collect();

    let odds: Vec<i32> = numbers
        .iter()
        .filter(|number| *number % 2 != 0)
        .copied()
        .collect();

    println!("{evens:?}");
    println!("{odds:?}");

    let first_even = numbers.into_iter().find(|number| number % 2 == 0);
    println!("{first_even:?}");

    let last_even = numbers.into_iter().rfind(|number| number % 2 == 0);
    println!("{last_even:?}");

    let channels = [
        TVChannel {
            name: String::from("CBS"),
            channel_type: ChannelType::Comedy,
        },
        TVChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
        TVChannel {
            name: String::from("NBC"),
            channel_type: ChannelType::News,
        },
        TVChannel {
            name: String::from("RustTV"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
    ];

    let good_channels: Vec<&TVChannel> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .collect();

    println!("{good_channels:#?}");

    let good_channels_names: Vec<String> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .map(|channel| channel.name.clone())
        .collect();

    println!("{good_channels_names:?}");

    let good_channel = channels
        .iter()
        .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

    println!("{good_channel:?}");

    separator("Any and All adapters");

    println!("{}", good_channels.len() == channels.len());

    println!("{}", good_channel.is_some());

    let all_are_rust = good_channels
        .iter()
        .all(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

    println!("Are all rust? {}", all_are_rust);

    let any_are_rust = channels
        .iter()
        .any(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

    println!("Any are rust? {}", any_are_rust);

    separator("Cloned (Heap based) adapter");

    let teas = [
        "Hot Earl Grey".to_string(),
        "Iced Green".to_string(),
        "Hot Matcha".to_string(),
    ];

    let more_teas: Vec<String> = teas
        .iter()
        .filter(|tea| tea.contains("Hot"))
        .cloned()
        .collect();

    println!("{more_teas:?}");

    separator("filterMap adapter");

    let stocks = ["nvda", "", "aapl", "msft", "goog"];

    let capitalized_stocks = stocks
        .iter()
        .filter(|stock| !stock.is_empty())
        .map(|stock| stock.to_uppercase())
        .collect::<Vec<String>>();

    println!("{capitalized_stocks:?}");

    let capitalized_stocks: Vec<String> = stocks
        .iter()
        .filter_map(|stock| {
            if stock.is_empty() {
                None
            } else {
                Some(stock.to_uppercase())
            }
        })
        .collect();

    println!("{capitalized_stocks:?}");

    separator("flatten adapter");

    let spreadsheet = vec![[100, 200, 300], [1, 2, 3], [4, 5, 6]];

    let values = spreadsheet.into_iter().flatten().collect::<Vec<i32>>();

    println!("{values:?}");

    separator("flatMap adapter");

    let attendees = [
        "Bob, Mary, Kevin",
        "Mike, Robbie, Matt, Austin",
        "Piers, Liam",
    ];

    let attendees: Vec<&str> = attendees
        .iter()
        .map(|group| group.split(", "))
        .flatten()
        .collect();

    println!("{attendees:?}");

    let attendees: Vec<&str> = attendees
        .iter()
        .flat_map(|group| group.split(", "))
        .collect();

    println!("{attendees:?}");

    separator("enumerate adapter");

    let applicants = vec!["Rob", "Bob", "Cob", "Alex", "Piers", "John", "Dan"];

    let winners = applicants
        .into_iter()
        .enumerate()
        .filter_map(|(index, applicant)| {
            if index % 3 == 0 {
                Some(applicant)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();

    println!("{winners:?}");

    separator("partition adapter");

    let numbers = [4, 8, 15, 16, 23, 42];

    let (evens, odds): (Vec<i32>, Vec<i32>) =
        numbers.into_iter().partition(|number| number % 2 == 0);

    println!("Evens: {evens:?}");
    println!("Odds: {odds:?}");

    separator("zip adapter");

    let first_names = ["Casey", "Robert", "Cargo", "Dan"];

    let last_names = ["Johnson", "Smith", "Rustman"];

    for (first_name, last_name) in first_names.iter().zip(last_names) {
        println!("{first_name} {last_name}")
    }

    let complete_names = first_names
        .iter()
        .zip(last_names)
        .map(|(first_name, last_name)| format!("{first_name} {last_name}"))
        .collect::<Vec<String>>();

    println!("{complete_names:?}");

    separator("fold adapter");

    let earnings = [4, 7, 9, 13];

    let sum = earnings.into_iter().fold(0, |total, current| {
        println!("Total: {total}, current: {current}");
        total + current
    });

    println!("sum: {sum}");

    let week = [
        SupportStaff {
            day: "Monday".to_string(),
            employee: "Brian".to_string(),
        },
        SupportStaff {
            day: "Tuesday".to_string(),
            employee: "cam".to_string(),
        },
        SupportStaff {
            day: "Wednesday".to_string(),
            employee: "Walter".to_string(),
        },
    ];

    let weekMap = week.into_iter().fold(HashMap::new(), |mut map, entry| {
        map.insert(entry.day, entry.employee);
        map
    });

    println!("{weekMap:?}");

    separator("reduce adapter");

    let earnings = [4, 7, 9, 13];

    let sum = earnings
        .into_iter()
        .reduce(|total, current| total + current);

    println!("{sum:?}");

    let earnings: [i32; 0] = [];
    let sum = earnings
        .into_iter()
        .reduce(|total, current| total + current);

    println!("{sum:?}");

    let address_portions = [
        "123 Elm Street".to_string(),
        "Suburbia".to_string(),
        "New Jersey".to_string(),
    ];

    println!("{}", address_portions.join(", "));

    let address = address_portions.into_iter().reduce(|mut acc, portion| {
        acc.push_str(", ");
        acc.push_str(&portion);
        acc
    });

    println!("{address:?}");
}

struct SupportStaff {
    day: String,
    employee: String,
}
