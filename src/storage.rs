use std::error::Error;
pub trait Storage {
    fn load(path: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    fn save(&self, path: &str) -> Result<(), Box<dyn Error>>;
}
