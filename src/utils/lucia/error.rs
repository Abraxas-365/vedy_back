use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database connection error: {0}")]
    DatabaseConnectionError(String),

    #[error("User session not found")]
    UserSessionNotFound,

    #[error("Invalid session ID")]
    InvalidSessionId,

    #[error("Session expired")]
    SessionExpired,

    #[error("Database query error: {0}")]
    DatabaseQueryError(String),

    #[error("User session table does not exist")]
    UserSessionTableNotExist,

    #[error("Auth user table does not exist")]
    AuthUserTableNotExist,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Duplicate user error: {0}")]
    DuplicateUserError(String),

    #[error("Session creation failed")]
    SessionCreationFailed,

    #[error("Session deletion failed")]
    SessionDeletionFailed,

    #[error("User creation failed")]
    UserCreationFailed,

    #[error("User update failed")]
    UserUpdateFailed,

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Unexpected error occurred: {0}")]
    UnexpectedError(String),
}
