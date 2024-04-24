#[derive(Debug)]
pub enum DatabaseErrorType {
    NotFound(String, String),
    ConnectionError(String, String),
    QueryError(String, String),
}

#[derive(Debug)]
pub enum SuccessResultType {
    Created(u64, u64),
    CreatedCollection(Vec<u64>),
    Updated(u64, u64),
    Deleted(u64, u64),
}

#[derive(Debug)]
pub enum EntityResult<T> {
    Success(T),
    Error(DatabaseErrorType),
}
