#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.20;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

pub fn run() {
    let mut flight = Flight::new(
        String::from("Kashmir"),
        String::from("Bangalore"),
        100.00,
        2,
    );

    flight.change_destination(String::from("Kerala"));
    flight.increase_price();
    flight.itinerary();

    println!("{flight:#?}");

    let another_flight = Flight {
        origin: String::from("Kashmir"),
        destination: String::from("Bangalore"),
        ..flight
    };
    println!("{another_flight:#?}");
}
