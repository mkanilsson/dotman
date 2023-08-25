use serde::Deserialize;

use crate::remote::Remote;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Source {
    pub remote: Remote,
    pub path: String,
}
