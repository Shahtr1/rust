pub fn double_the_length<T>(items: &Vec<T>) -> usize {
    items.len() * 2
}

pub fn last_two<T>(items: &[T]) -> &[T] {
    &items[items.len() - 2..]
}

pub fn first_five<'a>(text: &'a str, anouncement: &str) -> &'a str {
    println!("{anouncement:?}");
    &text[..5]
}

pub fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &str) -> &'a str {
    if first.contains(target) {
        first
    } else {
        second
    }
}
