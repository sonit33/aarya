use thiserror::Error;

#[derive(Error, Debug)]
pub enum AaryaUtilError {
    // file errors
    #[error("Failed to open file: [{1}] due to: [{0}]")]
    FileOpenError(String, String),
    #[error("Failed to read file: [{1}] due to: [{0}]")]
    FileReadError(String, String),
    // schema errors
    #[error("Schema failed to compile: [{1}] due to: [{0}]")]
    SchemaCompilationError(String, String),
    #[error("Validation failed for the file: [{0}]")]
    ValidationError(String),
    // web errors
    #[error("Formating failed at [{0}] due to [{1}]")]
    FormatError(String, String),
    #[error("Web request failed due to [{0}]")]
    ResponseError(String),
    #[error("Web request failed due to [{0}]")]
    PostRequestError(String),
    #[error("File write failed due to [{0}]")]
    // file errors
    FileWriteError(String, String),
    #[error("File creation failed due to [{0}]")]
    FileCreateError(String, String),
    #[error("Failed to parse JSON [{0}]")]
    JsonParseError(String, String),
    // database errors
}
