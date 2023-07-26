#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("DbError: {0}")]
    DbError(String),
    #[error("already slot exist.")]
    Conflicts,
}
