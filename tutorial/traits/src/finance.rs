pub trait Investment<T> {
    fn amount(&self) -> T; // Getter

    fn set_amount(&mut self, new_amount: T); // Setter

    fn double_amount(&mut self);
}

pub trait Taxable: Investment<f64> {
    // Investment is a super trait now
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
pub struct Income {
    pub amount: f64,
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }

    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}

impl Taxable for Income {}

pub struct Bonus {
    pub value: f64,
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_value: f64) {
        self.value = new_value;
    }

    fn double_amount(&mut self) {
        self.value *= 2.0;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;
}

pub struct QualityTime {
    pub minutes: u32,
}

impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }

    fn set_amount(&mut self, new_minutes: u32) {
        self.minutes = new_minutes;
    }

    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}
