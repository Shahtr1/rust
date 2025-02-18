pub trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn amount(&self) -> f64; // Getter

    fn set_amount(&mut self, new_amount: f64); // Setter

    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
pub struct Income {
    pub amount: f64,
}

impl Taxable for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

pub struct Bonus {
    pub value: f64,
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;

    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount;
    }
}
