pub trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64;
}

#[derive(Debug)]
pub struct Income {
    pub amount: f64,
}

impl Taxable for Income {
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}

pub struct Bonus {
    pub amount: f64,
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}
