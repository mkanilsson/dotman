use std::{env::var, fs, io};

use crate::{
    errors::{DotManResult, Error},
    remote::Remote,
    utils,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    remote: Remote,
    path: String,

    pub git: GitConfig,
}

impl Config {
    pub fn load() -> DotManResult<Config> {
        let content = match fs::read_to_string(utils::expand("", "~/.config/dotman/config.toml")?) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Err(Error::ConfigFileNotFound),
            Err(e) => return Err(e.into()),
            Ok(c) => c,
        };

        Ok(toml::from_str(&content)?)
    }

    pub fn url(&self) -> String {
        self.remote.url()
    }

    pub fn path(&self) -> DotManResult<String> {
        utils::expand("", &self.path)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GitConfig {
    #[serde(default = "GitUpdateType::default")]
    pub update_type: GitUpdateType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GitUpdateType {
    FetchRebase,
    Pull,
}

impl GitUpdateType {
    pub fn default() -> GitUpdateType {
        GitUpdateType::FetchRebase
    }
}
