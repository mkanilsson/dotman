use std::string::FromUtf8Error;

use crate::print;

pub enum GitError {
    NotARepository(String),
    Unknown(String),
}

pub enum Error {
    ConfigFileNotFound(String),
    MissingHomeVariable,

    Parse(toml::de::Error),
    IO(std::io::Error),
    Utf8(FromUtf8Error),

    Git(GitError),
    RemoteNotFound(String),
    MalformattedPackage(String),
    MalformattedPackageWithError(String, toml::de::Error),
    DependencyMissing(String, String), // Package, Missing dependency
    UnknownPackage(String),
}

impl Error {
    fn print(&self, func: fn(&str) -> ()) {
        match self {
            Error::ConfigFileNotFound(message) => {
                func(&format!("Config file not found...\n\t{}", message))
            }
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
            Error::DependencyMissing(p, dep) => func(&format!(
                "'{dep}' can't be found but is required by '{p}'..."
            )),
            Error::UnknownPackage(p) => func(&format!("'{p}' can't be found...")),
        }
    }

    pub fn print_info(&self) {
        self.print(print::info)
    }

    pub fn print_warning(&self) {
        self.print(print::info)
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

pub type DotManResult<T> = Result<T, Error>;
