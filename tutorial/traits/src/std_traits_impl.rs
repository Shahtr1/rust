use std::fmt::{Debug, Display, Formatter, Result};

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
