use std::fs;

use crate::{
    config::Config,
    errors::{DotManResult, Error, GitError},
    gitactions::GitWrapper,
    package::{Collection, Package, PackageFile},
};

#[derive(Debug)]
pub struct Repository {
    pub packages: Vec<Package>,
    pub collections: Vec<Collection>,
}

impl Repository {
    pub fn load(config: &Config) -> DotManResult<Repository> {
        // TODO: Implement --force
        let git = match GitWrapper::open(&config.url(), &config.path()?) {
            Ok(wrapper) => wrapper,
            Err(Error::Git(GitError::NotARepository(_))) => {
                match GitWrapper::clone(&config.url(), &config.path()?) {
                    Ok(wrapper) => wrapper,
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        };

        let dir = std::fs::read_dir(git.path())?;
        let mut packages = vec![];
        let mut collections = vec![];

        for path in dir {
            let path = path?;
            let file_name = path.file_name();
            let name = file_name.to_string_lossy();
            // TODO: Only get filename without rest of the path
            if name.ends_with(".toml") {
                let content = fs::read_to_string(path.path())?;
                let file: PackageFile = match toml::from_str(&content) {
                    Ok(f) => f,
                    Err(e) => return Err(Error::MalformattedPackageWithError(name.to_string(), e)),
                };
                if !file.valid() {
                    return Err(Error::MalformattedPackage(format!("{name}")));
                }

                if file.is_package() {
                    packages.push(file.package());
                }

                if file.is_collection() {
                    collections.push(file.collection());
                }
            }
        }

        return Ok(Repository {
            packages,
            collections,
        });
    }

    // This should only be called when the package is known to exist
    pub fn get_package_from_string(&self, name: &str) -> Package {
        for pkg in &self.packages {
            if pkg.name == name {
                return pkg.clone();
            }
        }

        unreachable!();
    }
}
