use std::{env::var, fs};

use crate::{
    errors::{DotManResult, Error},
    remote::Remote,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    remote: Remote,
    path: String,

    git: GitConfig,
}

impl Config {
    pub fn load() -> DotManResult<Config> {
        let home_path = match var("HOME") {
            Ok(path) => Ok(path),
            Err(_) => Err(Error::MissingHomeVariable),
        }?;

        let content = fs::read_to_string(format!("{}/.config/dotman/config.toml", home_path))?;

        Ok(toml::from_str(&content)?)
    }

    pub fn url(&self) -> String {
        self.remote.url()
    }

    pub fn path(&self) -> DotManResult<String> {
        let home = match var("HOME") {
            Ok(h) => h,
            Err(_) => return Err(Error::MissingHomeVariable),
        };

        Ok(self.path.replace("$HOME", &home))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GitConfig {
    #[serde(default = "GitUpdateType::default")]
    update_type: GitUpdateType,
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
