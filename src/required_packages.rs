use std::collections::HashSet;

use crate::package::PackageFile;

pub fn gather_required_packages(
    packages: &Vec<String>,
    repo: &Vec<PackageFile>,
) -> HashSet<String> {
    // TODO: return Result
    // let mut visited = vec![];
    HashSet::new()
}
