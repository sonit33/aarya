use thiserror::Error;

#[derive(Error, Debug)]
pub enum AaryaUtilError {
    #[error("Failed to open file: [{1}] due to: [{0}]")]
    FileOpenError(String, String),
    #[error("Failed to read file: [{1}] due to: [{0}]")]
    FileReadError(String, String),
    #[error("Schema failed to compile: [{1}] due to: [{0}]")]
    SchemaCompilationError(String, String),
    #[error("Validation failed for the file: [{0}]")]
    ValidationError(String),
    #[error("Formating failed at [{0}] due to [{1}]")]
    FormatError(String, String),
    #[error("Web request failed due to [{0}]")]
    ResponseError(String),
    #[error("Web request failed due to [{0}]")]
    PostRequestError(String),
}
