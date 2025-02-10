#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
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
}
