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
