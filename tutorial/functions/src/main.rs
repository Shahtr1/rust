#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
    open_store("Kashmir");
    open_store_2("also worksaaa");

    let result = (); // this is a unit, it is a default return value

    let multiplier = 3;

    let calculation = {
        let value = 5 + 4;
        value * multiplier
    };
}
fn open_store(neighbourhood: &str) -> i32 {
    return 2;
}
fn open_store_2(neighbourhood: &str) -> i32 {
    100
}

fn mystery() {}
fn mystery_2() -> () {}
