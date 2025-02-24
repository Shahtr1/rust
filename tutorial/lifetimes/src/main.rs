fn main() {
    let dog = String::from("Watson"); // referrant or lender
    {
        let my_pet = &dog;
        println!("{my_pet}")
    }

    println!("{dog}");

    {
        let my_pet = &dog;
        println!("{my_pet}")
    }
}
