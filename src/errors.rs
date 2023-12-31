use std::string::FromUtf8Error;

use inquire::InquireError;

use crate::print;

#[derive(Debug)]
pub enum GitError {
    NotARepository(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum Error {
    ConfigFileNotFound,
    MissingHomeVariable,

    Parse(toml::de::Error),
    IO(std::io::Error),
    Utf8(FromUtf8Error),
    Inquire(InquireError),

    Git(GitError),
    RemoteNotFound(String),
    MalformattedPackage(String),
    MalformattedPackageWithError(String, toml::de::Error),
    UnknownPackage(String),
}

impl Error {
    fn print(&self, func: fn(&str) -> ()) {
        match self {
            Error::ConfigFileNotFound => func("Config file not found..."),
            Error::Parse(e) => func(&format!("Parse error...\n\t{}", e.message())),
            Error::IO(e) => func(&format!("IO error...\n\t{}", &e.to_string())),
            Error::Git(e) => match e {
                GitError::NotARepository(message) => func(message),
                GitError::Unknown(message) => func(message),
            },
            Error::MissingHomeVariable => func("HOME environment variable isn't set"),
            Error::MalformattedPackageWithError(name, err) => {
                func(&format!("'{name}' is malformatted...\n\t{}", err.message()))
            }
            Error::MalformattedPackage(name) => func(&format!("'{name}' is malformatted...")),
            Error::Utf8(e) => func(&e.to_string()),
            Error::RemoteNotFound(message) => func(message),
            Error::UnknownPackage(p) => func(&format!("Package '{p}' can't be found...")),
            Error::Inquire(e) => func(&format!("Something went wrong with inquire...\n\t{}", e)),
        }
    }

    pub fn print_error(&self) {
        self.print(print::error)
    }

    pub fn print_fatal(&self) {
        self.print(print::fatal)
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::Parse(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<InquireError> for Error {
    fn from(value: InquireError) -> Self {
        Self::Inquire(value)
    }
}

pub type DotManResult<T> = Result<T, Error>;
