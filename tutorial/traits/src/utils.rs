use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

pub trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

pub trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
pub struct Hotel<T> {
    pub name: T,
    pub reservations: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

#[derive(Debug)]

pub struct AirBnB {
    pub host: String,
    pub guests: Vec<(String, u32)>,
}

impl AirBnB {
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

pub fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1);
}

pub fn book_for_one_night_gen<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

pub fn mix_and_match(
    first: &mut (impl Accommodation + Description),
    second: &mut impl Accommodation,
    guest: &str,
) {
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 1);
}

pub fn mix_and_match_gen<T: Accommodation + Description, U: Accommodation>(
    first: &mut T,
    second: &mut U,
    guest: &str,
) {
    first.book(guest, 1);
    first.get_description();

    second.book(guest, 1);
}

pub fn mix_and_match_gen_where<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    first.get_description();

    second.book(guest, 1);
}

pub fn choose_best_place_to_stay() -> impl Accommodation + Description + Debug {
    let likes_luxury = true;
    if likes_luxury {
        Hotel::new("Lux Continental")
    } else {
        Hotel::new("Continental")
    }
}
