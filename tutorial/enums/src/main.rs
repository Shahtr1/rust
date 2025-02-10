mod project;

#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
}

#[derive(Debug)]
enum Milk2 {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious, 2% milk is my favourite");
            }
            Milk::Lowfat(percent) => {
                println!("You've got lowfat {percent}% version");
            }
            Milk::Whole => {
                println!("You've got the whole milk!")
            }
        }
    }
}

#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            // OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
            //     println!("Your item is being packed for shipment");
            // }
            OnlineOrderStatus::Delivered => {
                println!("Your item has been delivered");
            }
            // _ => {
            //     println!("Your item is not there yet");
            // }
            other_status => {
                println!("Your item is {other_status:#?}");
            }
        }
    }
}

#[derive(Debug)]
enum OS {
    Windows,
    MacOS,
    Linux,
}

#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestarauntItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl(Meat),
    VeganPlate,
}

#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String), // this is called tuple variance
    DebitCard(String),
    Paypal(String, String), // Paypal(String, String) will require 48 bytes, 28 bytes * 2 in stack
    GPay { username: String, password: String },
}

#[derive(Debug)]
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Runnign a laundry with cold temperature")
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with a temperature of {temperature}")
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with a delicte cycle for {fabric_type}")
            }
        }
    }
}

// fn wash_laundry(cycle: LaundryCycle) {
//     match cycle {
//         LaundryCycle::Cold => {
//             println!("Runnign a laundry with cold temperature")
//         }
//         LaundryCycle::Hot { temperature } => {
//             println!("Running the laundry with a temperature of {temperature}")
//         }
//         LaundryCycle::Delicate(fabric_type) => {
//             println!("Running the laundry with a delicte cycle for {fabric_type}")
//         }
//     }
// }

fn years_since_release(os: OS) -> u32 {
    match os {
        OS::Windows => {
            println!("Quite old OS");
            39
        }
        OS::Linux => 34,
        OS::MacOS => 23,
    }
}

fn main() {
    let first_card = CardSuit::Hearts;
    let mut second_card = CardSuit::Spades;
    second_card = CardSuit::Clubs;
    println!("{:?}", second_card);

    let visa = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));
    let mastercard = PaymentMethodType::DebitCard(String::from("1234-5678-3456-0987"));
    println!("{:?}", visa);
    println!("{:?}", mastercard);

    let mut myPaymentMethod = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));
    myPaymentMethod =
        PaymentMethodType::Paypal(String::from("bob@email.com"), String::from("password"));

    println!("{:?}", myPaymentMethod);

    let gpayCredentials = Credentials {
        username: String::from("bob@email.com"),
        password: String::from("password"),
    };

    let gpay = PaymentMethodType::GPay {
        username: String::from("bob@email.com"),
        password: String::from("password"),
    };

    println!("{:?}", gpay);

    let lunch = RestarauntItem::Burrito {
        meat: Meat::Steak,
        beans: Beans::Pinto,
    };
    let dinner = RestarauntItem::Bowl(Meat::Chicken);
    let abandoned_meal = RestarauntItem::VeganPlate;

    println!("Lunch was {lunch:?} and dinner was {dinner:?}");
    println!("Nobody ate {abandoned_meal:?}");

    let my_computer = OS::MacOS;
    let years = years_since_release(my_computer);
    println!("My system is {years} years old.");

    let dads_computer = OS::Windows;
    let years = years_since_release(dads_computer);
    println!("My dad's computer is {years} years old.");

    LaundryCycle::Cold.wash_laundry();
    LaundryCycle::Hot { temperature: 92 }.wash_laundry();
    LaundryCycle::Delicate(String::from("Silk")).wash_laundry();

    OnlineOrderStatus::Delivered.check();
    OnlineOrderStatus::Packed.check();

    Milk::Lowfat(2).drink();
    Milk::Lowfat(40).drink();
    Milk::Whole.drink();

    let my_beverage = Milk2::NonDairy {
        kind: String::from("Oat"),
    };

    if let Milk2::Whole = my_beverage {
        println!("You have whole milk")
    }

    if let Milk2::Lowfat(percent) = my_beverage {
        println!("You beverage is {percent}% milk")
    }

    if let Milk2::NonDairy { kind } = my_beverage {
        println!("Your beverage is {kind} milk")
    } else {
        println!("You have some other milk variant")
    }

    let my_beverage = Milk2::Lowfat(2);

    let Milk2::Lowfat(percent) = my_beverage else {
        println!("You do not have the lowfat milk");
        return;
    };

    println!("percent is {percent}");

    project::run();
}
