pub trait MyAdd<Rhs = Self> {
    type Output;

    fn my_add(self, rhs: Rhs) -> Self::Output;
}

impl MyAdd for i32 {
    type Output = i32;

    fn my_add(self, rhs: i32) -> i32 {
        self + rhs
    }
}

impl MyAdd for f64 {
    type Output = f64;

    fn my_add(self, rhs: f64) -> f64 {
        self + rhs
    }
}
