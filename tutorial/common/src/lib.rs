pub fn separator(heading: &str) {
    let length = heading.len();
    let dot_length = 40;
    if length > dot_length - 2 {
        eprintln!(
            "Separator length should be less or equal to {}",
            dot_length - 2
        );
        return;
    }

    let available_length = dot_length - length;
    let left_length = available_length / 2;
    println!("{}", "*".repeat(dot_length));
    for i in 0..=available_length {
        if i == left_length {
            print!("{}", heading);
            continue;
        }
        print!("*");
    }
    println!("");
    println!("{}", "*".repeat(dot_length));
}
