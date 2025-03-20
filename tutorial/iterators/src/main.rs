use common::separator;
use problem_1::{count_chars, count_words};

mod adapters;
mod consuming_adapters;
mod manual;
mod problem_1;
mod traits;

fn main() {
    manual::iteration();
    traits::iteration();
    separator("Iteration problem");
    println!(
        "{:?}",
        count_words("Sally sells sea shells by the sea shore")
    );
    println!(
        "{:?}",
        count_chars("Sally sells sea shells by the sea shore")
    );
    adapters::run();
    consuming_adapters::run();
}
