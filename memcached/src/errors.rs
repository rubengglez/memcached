use std::{error::Error, fmt};

use config::ConfigError;

#[derive(Debug)]
pub enum Errors {
    InvalidNumberArguments(String),
    InvalidOptionalArguments(String),
    InvalidGivenPort(String),
    ConfigDataParseError(ConfigError),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Errors {}

impl From<ConfigError> for Errors {
    fn from(error: ConfigError) -> Self {
        Errors::ConfigDataParseError(error)
    }
}
