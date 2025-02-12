mod project;
fn main() {
    let mut pizza_diameters = vec![8, 10, 12, 14];
    // let pizza_diameters = Vec::<i32>::new();

    println!("{pizza_diameters:?}");

    // let pastas: Vec<&str> = Vec::new();
    let pastas: Vec<&str> = vec!["Good", "Bad", "Worst"];
    println!("{pastas:?}");

    pizza_diameters.push(16);
    pizza_diameters.push(18);

    pizza_diameters.insert(0, 4);

    let last_pizza_diameter = pizza_diameters.pop();

    match last_pizza_diameter {
        Option::Some(value) => println!("popped {}", value),
        Option::None => println!("vector is empty"),
    }

    println!("before popping all {pizza_diameters:?}");

    while let Some(value) = pizza_diameters.pop() {
        println!("popped {}", value);
    }

    println!("{pizza_diameters:?}");

    pizza_diameters = vec![8, 10, 12, 14, 20, 34, 87, 90];

    let third_diameter_from_start = pizza_diameters.remove(2);
    println!("Removed {}", third_diameter_from_start);
    println!("{pizza_diameters:?}");

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let mut pizza_toppings = vec![pepperoni, mushroom, sausage];
    // println!("{pepperoni:?}"); // ownership moved

    let reference = &pizza_toppings[2];
    println!("{reference}");

    let pizza_slice = &pizza_diameters[1..3];
    println!("{pizza_slice:?}");

    let option = pizza_toppings.get(2);

    let mut delicous_toppings = pizza_toppings;

    let topping_reference = &delicous_toppings[1];

    println!("The topping is {}", &topping_reference);

    delicous_toppings.push("Olives".to_string()); // sometimes it can point to deallocated memory, if we push

    // println!("The topping is {topping_reference}"); // not allowed

    delicous_toppings[1] = String::from("Cream");
    println!("{delicous_toppings:#?}");

    let target_topping = &mut delicous_toppings[2];
    target_topping.push_str(" and Meatballs");
    let another_topping = &mut delicous_toppings[2];
    println!("{delicous_toppings:#?}");

    let mut seasons: Vec<&str> = Vec::with_capacity(5);
    println!("Length: {}, Capaity: {}", seasons.len(), seasons.capacity());

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");

    println!("Length: {}, Capaity: {}", seasons.len(), seasons.capacity());

    seasons.push("Other");
    println!("Length: {}, Capaity: {}", seasons.len(), seasons.capacity());

    project::run();
}
