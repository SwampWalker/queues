use thiserror::Error;

use crate::queues::QueueError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("File does not exist.")]
    FileDoesNotExist(#[from] std::io::Error),
    #[error("File is not the expected format")]
    FormatError(#[from] QueueError),
    #[error("File is not the expected format")]
    LineFormatError(QueueError, String),
}
