
use crate::objects::validators::common_validator;
use crate::objects::validators::catalog_validator;
use crate::objects::traits::object_trait::ObjectTrait;
use crate::objects::errors::catalog_error::CatalogError;

#[derive(Debug)]
pub struct Catalog {
    // The name of the repository. This must be unique across all repositories and subrepositories.
    pub unique_name: String,
    // The type of object defined by the file. For catalog.yml, this must be catalog
    pub object_type: String,
    //The name of the repository, as it appears in the consumption tool. This value does not need to be unique.
    pub label: String ,
    //The version of SML being used.
    pub version: String,
    //Enables/disables aggressive aggregate promotion for the repository.
    // When enabled, all aggregates referenced by a query are considered for promotion,
    // regardless of whether a join to other non-preferred or non-aggregate datasets was required.
    pub aggressive_agg_promotion: bool,
    // Enables/disables speculative aggregates for the repository.
    // These are intended to improve the performance of queries from client BI tools faster than with demand-defined aggregates alone.
    // When enabled, the query engine automatically creates aggregate tables that it anticipates being useful based on your models.
    pub build_speculative_aggs: bool
}



impl ObjectTrait for Catalog{
    type Error = CatalogError;


    fn validate(&self) -> Result<(), CatalogError> {
        // Validate version
        common_validator::validate_version(&self.version).map_err(CatalogError::InvalidVersionFormat)?;

        // Validate unique name
        catalog_validator::validate_unique_name(&self.unique_name).map_err(CatalogError::InvalidUniqueName)?;

        // Validate object type
        catalog_validator::validate_object_type(&self.object_type).map_err(CatalogError::UnknownObjectType)?;

        // Validate label
        catalog_validator::validate_label(&self.label).map_err(|err| CatalogError::InvalidLabel(err))?;

        // Validate aggressive aggregate promotion
        catalog_validator::validate_aggressive_agg_promotion(self.aggressive_agg_promotion).map_err(|err| CatalogError::InvalidAggressiveAggPromotion(err))?;

        // Validate build speculative aggregates
        catalog_validator::validate_build_speculative_aggs(self.build_speculative_aggs).map_err(|err| CatalogError::InvalidBuildSpeculativeAggs(err))?;

        Ok(())
    }

    fn is_valid(&self) -> bool{
    true
    }
}

impl Catalog{
   pub fn new(
        unique_name: String,
        object_type: String,
        label: String,
        version: String,
        aggressive_agg_promotion: bool,
        build_speculative_aggs: bool
    ) -> Result<Self, CatalogError>{
        let catalog = Catalog {
            unique_name,
            object_type,
            label,
            version,
            aggressive_agg_promotion,
            build_speculative_aggs,
        };
        // Validate the Catalog object
        catalog.validate()?;

        Ok(catalog)
    }
}


#[cfg(test)]
mod tests {
    use super::*; // This brings the outer scope into the test module

    #[test]
    fn test_valid_catalog_creation() {
        let result = Catalog::new(
            "my_unique_name".to_string(),
            "catalog".to_string(),
            "My Catalog".to_string(),
            "1.0.0".to_string(),
            false,
            true,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_catalog_creation() {
        let result = Catalog::new(
            "".to_string(), // Invalid unique_name
            "catalog".to_string(),
            "My Catalog".to_string(),
            "1.0.0".to_string(),
            false,
            true,
        );
        assert!(result.is_err());
    }
}
