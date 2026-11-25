//! Failure paths for all of the entities
//! Mostly coupling [`sqlx::Error`] variants into personalized variants for easier handling.

use sqlx::error::ErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PoolErrorKind {
    #[error(
        "A 'Pool::acquire' timed out cause of too many conn being unavailable or too many erred when attempting new conn"
    )]
    PoolTimeOut,
    #[error("A 'Pool::close' called when waiting in 'Pool::acquire'")]
    PoolClosed,
}

#[derive(Error, Debug)]
pub enum IoErrorKind {
    #[error("Bg worker crashed")]
    WorkerCrash,
    #[error("Err from the 'sqlx::Any' driver mapping with native driver; msg = '{msg:?}'")]
    DriverError { msg: String },
    #[error("Err parsing conn string; msg = '{msg:?}'")]
    Config { msg: String },
    #[error("Err when attempt establish TLS conn; msg = '{msg:?}'")]
    Tls { msg: String },
    /// Includes sqlx::Error::Io(String)
    #[error("Err comm with backend; msg = '{msg:?}'")]
    OtherIoError { msg: String },
}

#[derive(Error, Debug)]
pub enum NotFoundKind {
    /// # Example
    /// ```
    /// let query = sqlx::raw_sql("UPDATE example_tbl SET name = (SELECT name FROM example_table WHERE id = 2) WHERE id = 3;");
    /// ```
    #[error("No row returned when expected at least 1")]
    Row,
    /// # Example
    /// ```
    /// let query = sqlx::raw_sql("SELECT non_existent_col FROM example_tbl;");
    /// ```
    #[error("Col: '{:?}' not found", .msg)]
    Column { msg: String },
    #[error("Type: '{:?}' does not exist", .typ)]
    Type { typ: String },
    #[error("Can't access col index: '{:?}'; Col has len: '{:?}'", .idx, .len)]
    ColumnIndexOutOfBounds { idx: usize, len: usize },
}

#[derive(Error, Debug)]
pub enum ConversionErrorKind {
    #[error("Err encoding val; msg = '{:?}'", .msg)]
    Encode { msg: String },
    #[error("Err decoding val; msg = '{:?}'", .msg)]
    Decode { msg: String },
    #[error("Err encoding a val (index = '{:?}') from the col = '{:?}'", .idx, .src)]
    ColumnDecode { idx: String, src: String },
}

#[derive(Error, Debug)]
pub enum EntityError {
    #[error("{0}")]
    IoError(IoErrorKind),
    #[error("Database constraint error: constraint = '{:?}', msg = '{msg}'", .constraint)]
    DatabaseConstraintError { constraint: ErrorKind, msg: String },
    #[error("{0}")]
    DataConversionError(ConversionErrorKind),
    #[error("{0}")]
    PoolError(PoolErrorKind),
    #[error("{0}")]
    MissingThingsError(NotFoundKind),
    #[error("Other error")]
    Other,
}

impl From<sqlx::Error> for EntityError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(database_error) => Self::DatabaseConstraintError {
                constraint: database_error.kind(),
                msg: String::from(database_error.message()),
            },
            sqlx::Error::Encode(msg) => Self::DataConversionError(ConversionErrorKind::Encode {
                msg: msg.to_string(),
            }),
            sqlx::Error::Decode(msg) => Self::DataConversionError(ConversionErrorKind::Decode {
                msg: msg.to_string(),
            }),
            sqlx::Error::ColumnDecode {
                index: idx,
                source: src,
            } => Self::DataConversionError(ConversionErrorKind::ColumnDecode {
                idx,
                src: src.to_string(),
            }),
            sqlx::Error::RowNotFound => Self::MissingThingsError(NotFoundKind::Row),
            sqlx::Error::ColumnNotFound(msg) => {
                Self::MissingThingsError(NotFoundKind::Column { msg })
            }
            sqlx::Error::TypeNotFound { type_name: typ } => {
                Self::MissingThingsError(NotFoundKind::Type { typ })
            }
            sqlx::Error::ColumnIndexOutOfBounds { index: idx, len } => {
                Self::MissingThingsError(NotFoundKind::ColumnIndexOutOfBounds { idx, len })
            }
            sqlx::Error::PoolTimedOut => Self::PoolError(PoolErrorKind::PoolTimeOut),
            sqlx::Error::PoolClosed => Self::PoolError(PoolErrorKind::PoolClosed),
            sqlx::Error::WorkerCrashed => Self::IoError(IoErrorKind::WorkerCrash),
            sqlx::Error::AnyDriverError(msg) => Self::IoError(IoErrorKind::DriverError {
                msg: msg.to_string(),
            }),
            sqlx::Error::Configuration(msg) => Self::IoError(IoErrorKind::Config {
                msg: msg.to_string(),
            }),
            sqlx::Error::Tls(msg) => Self::IoError(IoErrorKind::Tls {
                msg: msg.to_string(),
            }),
            sqlx::Error::Io(err) => Self::IoError(IoErrorKind::OtherIoError {
                msg: err.to_string(),
            }),
            _ => Self::Other,
        }
    }
}
