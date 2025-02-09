pub fn run() {
    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];
    let last_three = &mut cereals[2..];
    println!("{last_three:?}");
    last_three[2] = String::from("Lucky Charms");
    println!("{cereals:?}");

    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[..6];
    println!("{cookie}");
}
