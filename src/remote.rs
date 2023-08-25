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

impl Remote {
    pub fn url(&self) -> String {
        match &self {
            Remote::Github { owner, repo } => format!("git@github.com:{owner}/{repo}.git"),
            Remote::Gitlab { owner, repo } => format!("git@gitlab.com:{owner}/{repo}.git"),
            Remote::Custom { owner, repo, url } => format!("git@{url}:{owner}/{repo}.git"),
        }
    }
}
