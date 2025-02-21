use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter, Result},
    fs,
};

pub enum AppleType {
    RedDelicious,
    GrannySmiths,
}

impl Display for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(f, "🍎 Delicious"),
            AppleType::GrannySmiths => write!(f, "🍏 Granny Smith"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(f, "AppleType::RedDelicious"),
            AppleType::GrannySmiths => write!(f, "AppleType::Granny Smith"),
        }
    }
}

pub struct Apple {
    pub kind: AppleType,
    pub price: f64,
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} for {}", self.kind, self.price)
    }
}

impl Debug for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // write!(
        //     f,
        //     "Apple ::: [ Kind: {:?}, Price: {} ]",
        //     self.kind, self.price
        // )
        f.debug_struct("** Apple **")
            .field("Kind", &self.kind)
            .field("Price", &self.price)
            .finish()
    }
}

impl Drop for Apple {
    fn drop(&mut self) {
        let path = "traits/apple.txt";
        if fs::metadata(path).is_ok() {
            match fs::remove_file(path) {
                Ok(_) => println!("Goodbye, my sweet apple"),
                Err(err) => println!("Error deleting file: {}", err),
            }
        } else {
            // apple.txt does not exist, skipping deletion.
            // println!("apple.txt does not exist, skipping deletion.");
        }
    }
}

pub struct Appointment {
    pub doctor: String,
    pub start_time: String,
    pub end_time: String,
}

impl Appointment {
    pub fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

impl Clone for Appointment {
    fn clone(&self) -> Self {
        Self {
            doctor: self.doctor.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Duration {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl Duration {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

impl Copy for Duration {}

// #[derive(PartialEq)]
pub struct Flight {
    pub origin: String,
    pub destination: String,
    pub time: String,
}

impl Flight {
    pub fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

pub struct BusTrip {
    pub origin: String,
    pub destination: String,
    pub time: String,
}

impl BusTrip {
    pub fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

// #[derive(PartialEq)]
pub enum Musician {
    SingerSongwriter(String),
    Band(u32),
}

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::SingerSongwriter(name) => match other {
                Self::SingerSongwriter(other_name) => name == other_name,
                Self::Band(_) => false,
            },
            Self::Band(members) => match other {
                Self::SingerSongwriter(_) => false,
                Self::Band(other_members) => members == other_members,
            },
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Job {
    pub salary: u32,
    pub commute_time: u32,
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.salary.partial_cmp(&other.salary)

        // if self.salary == other.salary {
        //     Some(Ordering::Equal)
        // } else if self.salary < other.salary {
        //     Some(Ordering::Less)
        // } else if self.salary > other.salary {
        //     Some(Ordering::Greater)
        // } else {
        //     None
        // }
    }
}
