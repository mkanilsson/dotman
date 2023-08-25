use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "host")]
#[serde(rename_all = "kebab-case")]
pub enum Remote {
    Github {
        owner: String,
        repo: String,
    },
    Gitlab {
        owner: String,
        repo: String,
    },
    Custom {
        owner: String,
        repo: String,
        url: String,
    },
}
