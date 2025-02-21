use std::{
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
            println!("apple.txt does not exist, skipping deletion.");
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
