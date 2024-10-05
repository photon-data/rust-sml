
use crate::objects::validators::connection_validator;
use crate::objects::traits::object_trait::ObjectTrait;
use crate::objects::errors::connection_error::ConnectionError;

#[derive(Debug)]
pub struct Connection {
    // A unique name for the database and the schema. This must be unique across all repositories and subrepositories.
    pub unique_name: String,
    // The type of object defined by this file. For connections, this value must be connection.
    pub object_type: String,
    //The name of the database connection as it appears in the consumption tool. This value does not need to be unique.
    pub label: String ,
    //The name of the database connection itself, excluding the schema.
    pub as_connection: String,
    //The source database used for this connection.
    pub database: String,
   // The source schema used for this connection.
   pub schema: String,

}

impl ObjectTrait for Connection {
    type Error = ConnectionError;

    fn validate(&self) -> Result<(), ConnectionError> {
        // Validate unique name
        connection_validator::validate_unique_name(&self.unique_name)
            .map_err(ConnectionError::InvalidUniqueName)?;

        // Validate object type
        connection_validator::validate_object_type(&self.object_type)
            .map_err(ConnectionError::UnknownObjectType)?;

        // Validate label
        connection_validator::validate_label(&self.label)
            .map_err(|err| ConnectionError::InvalidLabel(err))?;

        // Validate as_connection
        connection_validator::validate_as_connection(&self.as_connection)
            .map_err(|err| ConnectionError::InvalidAsConnection(err))?;

        // Validate database
        connection_validator::validate_database(&self.database)
            .map_err(|err| ConnectionError::InvalidDatabase(err))?;

        // Validate schema
        connection_validator::validate_schema(&self.schema)
            .map_err(|err| ConnectionError::InvalidSchema(err))?;

        Ok(())
    }

}
impl Connection{
   pub fn new(
        unique_name: String,
        object_type: String,
        label: String,
        as_connection: String,
        database: String,
        schema: String
    ) -> Result<Self, ConnectionError>{
        let connection = Connection {
            unique_name,
            object_type,
            label,
            as_connection,
            database,
            schema
        };
        // Validate the Connection object
       connection.validate()?;

        Ok(connection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::errors::connection_error::ConnectionError;

    #[test]
    fn test_valid_connection_creation() {
        let result = Connection::new(
            "my_unique_connection".to_string(),
            "connection".to_string(),
            "My Connection".to_string(),
            "my_connection_string".to_string(),
            "my_database".to_string(),
            "my_schema".to_string(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_connection_creation_empty_unique_name() {
        let result = Connection::new(
            "".to_string(), // Invalid unique_name
            "connection".to_string(),
            "My Connection".to_string(),
            "my_connection_string".to_string(),
            "my_database".to_string(),
            "my_schema".to_string(),
        );
        assert!(result.is_err());
        if let Err(ConnectionError::InvalidUniqueName(err)) = result {
            assert_eq!(err, "A Connection's unique_name cannot be empty.");
        }
    }

    #[test]
    fn test_invalid_connection_creation_empty_label() {
        let result = Connection::new(
            "my_unique_connection".to_string(),
            "connection".to_string(),
            "".to_string(), // Invalid label
            "my_connection_string".to_string(),
            "my_database".to_string(),
            "my_schema".to_string(),
        );
        assert!(result.is_err());
        if let Err(ConnectionError::InvalidLabel(err)) = result {
            assert_eq!(err, "Label cannot be empty.");
        }
    }

    #[test]
    fn test_invalid_connection_creation_invalid_object_type() {
        let result = Connection::new(
            "my_unique_connection".to_string(),
            "invalid_type".to_string(), // Invalid object type
            "My Connection".to_string(),
            "my_connection_string".to_string(),
            "my_database".to_string(),
            "my_schema".to_string(),
        );
        assert!(result.is_err());
        if let Err(ConnectionError::UnknownObjectType(err)) = result {
            assert_eq!(err, "Object type must be 'connection'.");
        }
    }

    // Additional tests can go here for other validation scenarios
}
