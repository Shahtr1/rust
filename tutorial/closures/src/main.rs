fn main() {
    let multiplier = 5;

    let multiply_by = |value: i32| -> i32 { value * multiplier };

    println!("{}", multiply_by(5));

    let product = |a, b| a * b;
    println!("{}", product(5 as u8, 2));
    // println!("{}", product(1000, 2)); error as inferred type is u8

    let mut numbers = vec![4, 8, 15, 16, 23, 42];
    let print_numbers = || {
        println!("{:?}", numbers);
    };
    print_numbers();
    println!("{:?}", numbers);

    let mut add_number = || numbers.push(100);
    add_number();
    add_number();

    println!("{:?}", numbers);
}
