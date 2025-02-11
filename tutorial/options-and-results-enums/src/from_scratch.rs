#[derive(Debug, Copy, Clone)]
pub enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    // will not loose ownership, as we derive copy trait
    pub fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh"),
        }
    }
    pub fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}
