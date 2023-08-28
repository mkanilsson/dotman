use crate::{print, repo::Repository};
use colored::Colorize;

pub fn inspect(repo: &Repository, package: &str) {
    for pkg in &repo.packages {
        if pkg.name == package {
            pkg.pprint();
            return;
        }
    }

    for col in &repo.collections {
        if col.name == package {
            col.pprint();
            return;
        }
    }

    print::warning(&format!(
        "No package or collection named '{}' was found",
        package.bold()
    ));
}
