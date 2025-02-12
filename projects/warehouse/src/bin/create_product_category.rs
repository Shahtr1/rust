use fake::{Fake, Faker};

use warehouse::ProductCategory;

// Create a fictional Product Category
fn main() {
    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);
}
