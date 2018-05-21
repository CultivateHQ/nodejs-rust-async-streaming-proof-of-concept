use std::io;
use std::sync::mpsc::{RecvTimeoutError, SendError};

#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    RecvTimeoutError(RecvTimeoutError),
    SendError(SendError<()>),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> MyError {
        MyError::Io(error)
    }
}

impl From<RecvTimeoutError> for MyError {
    fn from(error: RecvTimeoutError) -> MyError {
        MyError::RecvTimeoutError(error)
    }
}

impl From<SendError<()>> for MyError {
    fn from(error: SendError<()>) -> MyError {
        MyError::SendError(error)
    }
}
