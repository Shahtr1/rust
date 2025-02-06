#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    let eight_bit: i8 = -112;
    let eight_bit = -112i8;

    let mut sixteen_bit_signed: i64 = 6_2_5;

    sixteen_bit_signed = (eight_bit as i64).pow(2);

    // will be 64 on 64-bit system
    let depending_on_arch: usize = 10;
    let depending_on_arch: isize = 10;

    let filepath = "   C:\\My Documents\\new\\videos   "; // string literal
    let filepath = r"C:\My Documents\new\videos"; // raw string

    println!("{},{}", eight_bit.abs(), filepath.trim());
    println!("{}", sixteen_bit_signed);

    let pi = 3.1415926535897932384;
    println!("The current value of pi is {pi:.4}");
    println!("The current value of pi is {:.5}", pi);

    let floor_division = 5 / 3;

    let decimal_division = 5.0 / 3.0;

    println!("floor_division: {floor_division}, decimal_division {decimal_division}");

    let mut year = 2025;
    year += 1;

    let emoji: char = '😂';

    println!("My emoji is {emoji}");

    let numbers = [4, 18, 16, 23, 42];
    let numbers: [i32; 6] = [4, 18, 16, 23, 42, 100];
    let currency_rates: [f64; 0] = [];

    let apples = ["Granny Smith", "McIntosh", "Red Delicious"];
    println!("Length : {}", apples.len());
    println!("{:?}", apples);

    // Debug trait representation
    println!("{apples:?}");

    // Debug trait representation
    println!("{apples:#?}"); // pretty print

    dbg!(2 + 2);
    dbg!(apples);

    let employee = ("Molly", 33, "Marketing");

    let name = employee.0;
    let age = employee.1;
    let department = employee.2;

    let (name, age, department) = employee;

    println!("{employee:#?}");
    dbg!(employee);

    let month_days: std::ops::Range<i32> = 1..31;
    let month_days_inclusive = 1..=31;

    dbg!(month_days.clone()); // debug without consuming
    dbg!(month_days_inclusive);

    for number in month_days {
        // dbg!(number);
    }

    let letters = 'b'..'f';
    for letter in letters {
        // dbg!(letter);
    }
}
