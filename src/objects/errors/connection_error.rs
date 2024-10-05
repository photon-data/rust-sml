#[derive(Debug)]
pub enum ConnectionError {
    InvalidUniqueName(String),
    UnknownObjectType(String),
    InvalidLabel(String),
    InvalidAsConnection(String),
    InvalidDatabase(String),
    InvalidSchema(String),


}
