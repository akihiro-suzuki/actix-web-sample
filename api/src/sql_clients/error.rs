use crate::domains::error::Error;

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Error {
        Error::DbError(err.to_string())
    }
}
