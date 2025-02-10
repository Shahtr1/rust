#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(self) {
        match self {
            Subscription::Free => println!("You have limited access to the site"),
            Subscription::Basic(price, months) => {
                println!(
                    "You have limited access to the site's premium features for {} for {} months",
                    { price },
                    { months }
                );
            }
            Subscription::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is {tier:?}"
                )
            }
        }
    }
}

pub fn run() {
    Subscription::Free.summarize();
    Subscription::Basic(499.99, 2).summarize();
    Subscription::Premium { tier: Tier::Gold }.summarize();
}
