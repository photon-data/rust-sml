/// Validates the unique name and returns it if valid.
pub fn validate_unique_name(unique_name: &str) -> Result<String, String> {
    if unique_name.is_empty() {
        return Err("A Connection's unique_name cannot be empty.".to_string());
    }
    Ok(unique_name.to_string()) // Return the validated unique name
}

/// Validates the object type and returns it if valid.
pub fn validate_object_type(object_type: &str) -> Result<String, String> {
    if object_type != "connection" {
        return Err("Object type must be 'connection'.".to_string());
    }
    Ok(object_type.to_string()) // Return the validated object type
}

/// Validates the label and returns it if valid.
pub fn validate_label(label: &str) -> Result<String, String> {
    if label.is_empty() {
        return Err("Label cannot be empty.".to_string());
    }
    Ok(label.to_string()) // Return the validated label
}

/// Validates the as_connection and returns it if valid.
pub fn validate_as_connection(as_connection: &str) -> Result<String, String> {
    if as_connection.is_empty() {
        return Err("as_connection cannot be empty.".to_string());
    }
    Ok(as_connection.to_string()) // Return the validated as_connection
}

/// Validates the database and returns it if valid.
pub fn validate_database(database: &str) -> Result<String, String> {
    if database.is_empty() {
        return Err("Database cannot be empty.".to_string());
    }
    Ok(database.to_string()) // Return the validated database
}

/// Validates the schema and returns it if valid.
pub fn validate_schema(schema: &str) -> Result<String, String> {
    if schema.is_empty() {
        return Err("Schema cannot be empty.".to_string());
    }
    Ok(schema.to_string()) // Return the validated schema
}
