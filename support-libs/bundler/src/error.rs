use std::io;
use std::result;
use std::env;
use rmps;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Env { message: String, err: Option<env::VarError> },
    PluginNotInitialized { plugin_type: &'static str, plugin_id: String },
    InvalidConfig { plugin_id: &'static str },
    DeserializeConfig { message: String },
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

impl From<rmps::decode::Error> for Error {
    fn from(other: rmps::decode::Error) -> Self {
        Error::DeserializeConfig { message: format!("Desereialize failed: {}", other) }
    }
}

pub type Result<T> = result::Result<T, Error>;