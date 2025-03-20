use common::separator;

pub fn run() {
    separator("Consuming adapters");
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let total: i32 = numbers.iter().sum();
    println!("{total}");

    let product: i32 = numbers.iter().product();
    println!("{product}");

    let numbers2 = Vec::<i32>::new();

    let max = numbers2.iter().max();
    println!("{max:?}");

    let min = numbers.iter().min();
    println!("{min:?}");

    let count = numbers.iter().count();
    println!("{count}");

    let numbers = vec![4.6, 8.8, 0.0 / 0.0, 6.2, f64::NAN];

    println!("{numbers:?}");

    let total: f64 = numbers.iter().sum();
    println!("Total is {total}");

    // let max = numbers.iter().max(); // error as Order trait not implemented for floats

    let total: f64 = numbers
        .iter()
        .filter(|number| !number.is_nan())
        .copied()
        .fold(0.0, |total, current| total + current);

    println!("{total}");

    let max: Option<f64> = numbers
        .iter()
        // .filter(|num| !num.is_nan()) // as max will ignore NaN
        .copied()
        .reduce(|acc, current| acc.max(current));

    println!("{max:?}");
}
