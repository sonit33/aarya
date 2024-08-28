#[derive(Debug)]
pub enum DatabaseErrorType {
    NotFound(String, String),
    ConnectionError(String, String),
    QueryError(String, String),
    MutationError(String, String),
}

#[derive(Debug)]
pub enum SuccessResultType {
    Created(String),
    Updated(String),
    Deleted(String),
}

#[derive(Debug)]
pub enum EntityResult<T> {
    Success(T),
    Error(DatabaseErrorType),
}
