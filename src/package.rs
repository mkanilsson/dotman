use serde::Deserialize;

use super::remote::Remote;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Package {
    pub name: String,
    pub description: String,
    pub repo: Remote,
    pub install_path: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Collection {
    pub name: String,
    pub description: String,
    pub packages: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PackageFile {
    #[serde(rename(deserialize = "Collection"))]
    pub collection: Option<Collection>,

    #[serde(rename(deserialize = "Package"))]
    pub package: Option<Package>,
    // TODO: Add maintainer
}

impl PackageFile {
    pub fn is_collection(&self) -> bool {
        self.collection.is_some()
    }

    pub fn is_package(&self) -> bool {
        self.package.is_some()
    }

    pub fn collection(&self) -> Collection {
        self.collection.as_ref().unwrap().clone()
    }

    pub fn package(&self) -> Package {
        self.package.as_ref().unwrap().clone()
    }

    pub fn valid(&self) -> bool {
        let both = !(self.is_collection() && self.is_package());
        let neither = !(!self.is_collection() && !self.is_package());
        both && neither
    }
}
