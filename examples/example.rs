// use serde_yml::{self, Value};
use rust_sml::objects::Catalog;

fn main() {
    // Example of creating a valid Catalog
    let unique_name = "my_unique_name";
    let object_type = "catalog";
    let label = "My Catalog";
    let version = "1.0.0";
    let aggressive_agg_promotion = false;
    let build_speculative_aggs = true;

    match Catalog::new(
        unique_name.to_string(),
        object_type.to_string(),
        label.to_string(),
        version.to_string(),
        aggressive_agg_promotion,
        build_speculative_aggs,
    ) {
        Ok(catalog) => println!("Catalog created successfully: {:?}", catalog),
        Err(err) => println!("Error creating catalog: {:?}", err),
    }


}