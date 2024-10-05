use rust_sml::objects::Connection;

fn main() {
    // Example of creating a valid Connection
    let valid_unique_name = "my_unique_connection";
    let object_type = "connection"; // Must be "connection"
    let valid_label = "My Connection";
    let valid_as_connection = "my_database_connection";
    let valid_database = "my_database";
    let valid_schema = "my_schema";

    match Connection::new(
        valid_unique_name.to_string(),
        object_type.to_string(),
        valid_label.to_string(),
        valid_as_connection.to_string(),
        valid_database.to_string(),
        valid_schema.to_string(),
    ) {
        Ok(connection) => println!("Connection created successfully: {:?}", connection),
        Err(err) => println!("Error creating connection: {:?}", err),
    }

}
