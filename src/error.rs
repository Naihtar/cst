use thiserror::Error;

/// Errors originating from domain rules and invariants.
#[derive(Error, Debug, PartialEq)]
pub enum DomainError {
    /// Task description is blank or whitespace-only.
    #[error("Task information is empty")]
    EmptyTaskInformation,
    /// Supplied ID is structurally invalid.
    #[error("Invalid ID")]
    InvalidID,
    /// No task exists with the given ID.
    #[error("ID not found: {0}")]
    NotFoundID(i64),
    /// Undo was requested but no snapshot is available.
    #[error("Nothing to undo")]
    NothingToUndo,
}

/// Errors produced during CLI argument parsing and terminal I/O.
#[derive(Error, Debug, PartialEq)]
pub enum CliError {
    /// No command flag was provided.
    #[error("No command provided (use uppercase like -A, -L)")]
    MissingCommand,
    /// The command character is not recognised.
    #[error("Unknown command: -{0}")]
    UnknownCommand(char),
    /// An argument that should be a numeric ID could not be parsed.
    #[error("Invalid ID format")]
    InvalidIdFormat,
    /// A modifier flag is not recognised.
    #[error("Unknown modifier: {0}")]
    InvalidModifier(char),
    /// A required argument is absent.
    #[error("Missing argument: {0}")]
    MissingArgument(char),
    /// `stdout` could not be flushed.
    #[error("Failed to flush the terminal output")]
    StdoutFlushError,
    /// Reading from `stdin` failed.
    #[error("Failed to read from the input stream")]
    StdinReadError,
    /// `stdin` was closed before input was received.
    #[error("The input stream was closed unexpectedly")]
    EmptyInputStream,
    /// The user entered an unrecognised confirmation string.
    #[error("Invalid confirmation: {0}")]
    InvalidConfirmation(String),
}

/// Errors related to application configuration and environment variables.
#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    /// A configuration value could not be parsed.
    #[error("Parse error: {0}")]
    ParseError(String),
    /// A required environment variable is not set.
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(&'static str),
    /// The language code is not supported.
    #[error("Invalid language: '{0}'")]
    InvalidLanguage(String),
    /// Settings were accessed before being initialised.
    #[error("Settings not initialized")]
    SettingsNotInitialized,
    /// Generic environment-related error.
    #[error("Environment error: {0}")]
    Env(String),
}

/// Errors that occur during file import or export.
#[derive(Error, Debug, PartialEq)]
pub enum IoError {
    /// A required field is missing in the source data.
    #[error("Missing field: {0}")]
    MissingField(String),
    /// The source data could not be parsed.
    #[error("Parse error: {0}")]
    ParseError(String),
    /// Serialization of output data failed.
    #[error("Serialization error: {0}")]
    SerializationError(String),
    /// A filesystem operation failed.
    #[error("File error: {0}")]
    FileError(String),
    /// Export was attempted with no tasks.
    #[error("No tasks to export")]
    EmptyExport,
    /// Import was attempted on an empty file.
    #[error("No tasks to import")]
    EmptyImport,
}

/// Top-level application error that wraps all sub-errors.
#[derive(Error, Debug)]
pub enum CSTError {
    #[error("Domain: {0}")]
    Domain(#[from] DomainError),
    #[error("CLI: {0}")]
    Cli(#[from] CliError),
    #[error("Config: {0}")]
    Config(#[from] ConfigError),
    #[error("IO: {0}")]
    Io(#[from] IoError),
    #[error("Database: {0}")]
    Rusqlite(#[from] rusqlite::Error),
    #[error("Config: {0}")]
    Dotenvy(#[from] dotenvy::Error),
    #[error("Serialization: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl From<std::io::Error> for CSTError {
    fn from(err: std::io::Error) -> Self {
        CSTError::Io(IoError::FileError(err.to_string()))
    }
}

impl From<serde_yaml::Error> for CSTError {
    fn from(err: serde_yaml::Error) -> Self {
        CSTError::Config(ConfigError::ParseError(err.to_string()))
    }
}

impl From<csv::Error> for CSTError {
    fn from(err: csv::Error) -> Self {
        CSTError::Io(IoError::ParseError(err.to_string()))
    }
}

impl From<rust_xlsxwriter::XlsxError> for CSTError {
    fn from(err: rust_xlsxwriter::XlsxError) -> Self {
        CSTError::Io(IoError::FileError(err.to_string()))
    }
}
