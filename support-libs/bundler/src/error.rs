use std::io;
use std::result;
use std::env;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Env { message: String, err: Option<env::VarError> },
    ActionTypeNotInitialized { type_id: String },
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub type Result<T> = result::Result<T, Error>;