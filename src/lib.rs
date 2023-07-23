use std::{
    error::Error,
    fmt::{Debug, Formatter, Result as FmtResult},
};

pub struct ErrorMessage(String);

impl ErrorMessage {
    pub fn new<S: Into<String>>(message: S) -> Self {
        ErrorMessage(message.into())
    }
}

impl Debug for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl<E: Error> From<E> for ErrorMessage {
    fn from(value: E) -> Self {
        ErrorMessage(format!("{}", value))
    }
}
