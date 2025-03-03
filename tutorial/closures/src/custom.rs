#[derive(Debug)]
pub struct Vault {
    pub password: String,
    pub treasure: String,
}

impl Vault {
    pub fn unlock<F>(self, procedure: F) -> Option<String>
    where
        F: FnOnce() -> String,
    {
        let user_password = procedure();
        if user_password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Location {
    pub name: String,
    pub treasure: u32,
}

#[derive(Debug)]
pub struct Map<'a> {
    pub locations: &'a [Location],
}

impl<'a> Map<'a> {
    pub fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        let final_index = self.locations.len() - 1;
        let mut current_index = 0;
        while current_index <= final_index {
            let current_location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}

pub fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    procedure();
    procedure();
    procedure();
}

pub fn bake_cake() {
    println!("Hello chocolate");
}
