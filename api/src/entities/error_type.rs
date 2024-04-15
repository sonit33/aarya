pub enum DatabaseErrorType {
    NotFound(String, String),
    ConnectionError(String, String),
    QueryError(String, String),
}

pub enum EntityResult<T> {
    Success(T),
    Error(DatabaseErrorType),
}
