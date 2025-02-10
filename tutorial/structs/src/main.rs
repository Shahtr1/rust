mod project;

#[derive(Debug)]
struct ShortDuration(u32, u32); // Hours, Minutes

#[derive(Debug)]
struct LongDuration(u32, u32); // Years, Months

// unit like struct
#[derive(Debug)]
struct Empty;

#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_hard_drive_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_hard_drive_capacity;
        self
    }
}

impl TaylorSwiftSong {
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }
}

impl TaylorSwiftSong {
    // Immutable struct value (self parameter takes ownership)
    fn display_song_info(self: Self) -> Self {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {}", self.duration_secs);
        self
    }

    // Mutable struct value
    fn double_length(mut self) -> Self {
        self.duration_secs = self.duration_secs * 2;
        self
    }

    // Immutable reference to struct instance
    fn display_song_info_ref(&self) {
        println!("Title: {}", self.title);
        println!("Year since release: {}", self.years_since_release());
        println!("Duration: {}", self.duration_secs);
    }

    // Mutable reference to struct instance
    fn double_length_ref(&mut self) {
        self.duration_secs = self.duration_secs * 2;
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        2024 - self.release_year
    }
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        is_hot,
        price,
    }
}

fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my {}", coffee.name);
    coffee.is_hot = false;
}

fn go_to_work(length: ShortDuration) {
    println!("Passing time {} hour(s) and {} minutes", length.0, length.1);
}

fn accept_tuple(length: (u32, u32)) {}

fn main() {
    let coffee = ("Caramel Macchito", 5.99, true); // no context in tuples

    let mocha = Coffee {
        name: String::from("Caramel Macchito"),
        price: 5.99,
        is_hot: true,
    };

    println!("{}", mocha.name);

    let favourite_coffee = mocha.name;
    println!("{favourite_coffee}");

    // println!("", mocha.name); // mocha is no longer owner of name field

    let mut beverage = Coffee {
        name: String::from("Caramel Macchito"),
        price: 5.99,
        is_hot: false,
    };

    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = true;

    let name = String::from("Latte");
    let latte = make_coffee(name, 4.99, true);
    println!("{}", latte.name);

    let mut caramel = Coffee {
        name: String::from("Caramel"),
        ..mocha // rust will not copy name field
    };
    drink_coffee(&mut caramel);
    println!("coffee is hot? {}", caramel.is_hot);

    println!("{caramel:?}");
    println!("{caramel:#?}");

    let mut song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 1939,
        duration_secs: 3600,
    };

    song = song.display_song_info();
    song = song.double_length();
    song = song.display_song_info();

    println!("{song:#?}");

    song.display_song_info_ref();
    song.double_length_ref();

    println!("{song:#?}");

    let blank_space = TaylorSwiftSong::new(String::from("some stupid song"), 1989, 0);

    let mut computer = Computer::new(String::from("M3 Max"), 64, 100);

    computer
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(4000)
        .upgrade_hard_drive_capacity(500000);

    println!("Stats: {computer:#?}");

    let work_shift = ShortDuration(8, 0);
    let era = LongDuration(5, 3);
    println!("{work_shift:?}");
    go_to_work(work_shift);
    // go_to_work(era);
    // accept_tuple(work_shift);

    let my_empty_struct = Empty;

    project::run();
}
