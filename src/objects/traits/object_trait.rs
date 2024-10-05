
pub trait ObjectTrait {
    type Error;
    fn is_valid(&self) -> bool {
        self.validate().is_ok() // Return true if validation passes, false otherwise
    }
    fn validate(&self)  -> Result<(), Self::Error>;
}