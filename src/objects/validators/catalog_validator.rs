

/// Validates the unique name and returns it if valid.

pub fn validate_unique_name(unique_name: &str) -> Result<String, String> {
    if unique_name.is_empty() {
        return Err("A Catalog's unique_name cannot be empty.".to_string());
    }
    Ok(unique_name.to_string()) // Return the validated unique name
}

/// Validates the object type and returns it if valid.
pub fn validate_object_type(object_type: &str) -> Result<String, String> {
    if object_type.is_empty() || object_type != "catalog" {
        return Err("A Catalog object_type must be of type 'catalog'.".to_string());
    }
    Ok(object_type.to_string()) // Return the validated object type
}

/// Validates the label and returns it if valid.
pub fn validate_label(label: &str) -> Result<String, String> {
    if label.is_empty() {
        return Err("A Catalog's label cannot be empty.".to_string());
    }
    Ok(label.to_string()) // Return the validated label
}

/// Validates aggressive aggregate promotion setting.
pub fn validate_aggressive_agg_promotion(aggressive: bool) -> Result<bool, String> {
    // In this case, we just return the value since it's a boolean
    Ok(aggressive)
}

/// Validates speculative aggregates setting.
pub fn validate_build_speculative_aggs(build: bool) -> Result<bool, String> {
    // In this case, we just return the value since it's a boolean
    Ok(build)
}