use crate::objects::errors::catalog_error::CatalogError;

pub trait ObjectTrait {
    fn is_valid(&self) -> bool {
        self.validate().is_ok() // Return true if validation passes, false otherwise
    }
    fn validate(&self)  -> Result<(), CatalogError>;
}