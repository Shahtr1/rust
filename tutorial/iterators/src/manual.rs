use common::separator;

pub fn iteration() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let mut current_index = 0;

    let final_index = numbers.len() - 1;

    separator("loop");
    loop {
        // can cause infinite loop, if we miss this
        if current_index > final_index {
            break;
        }

        println!("{}", numbers[current_index]);
        current_index += 1;
    }

    separator("while");

    current_index = 0;
    while current_index <= final_index {
        println!("{}", numbers[current_index]);
        current_index += 1; // can cause infinite loop, if we miss this
    }

    separator("for loop (Recommended)");

    for number in &numbers {
        println!("{}", number);
    }

    println!("{:?}", numbers);
}
