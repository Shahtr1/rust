use std::ops::Add;

#[derive(Debug)]
pub struct Lunch {
    pub cost: f64,
}

impl Add for Lunch {
    type Output = Lunch;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}

pub fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
