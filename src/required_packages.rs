use std::collections::HashSet;

use crate::{
    errors::{DotManResult, Error},
    repo::Repository,
};

// The amout of cloing in this file is sad... but performace isn't an issue
pub fn gather_required_packages(
    pcs: &Vec<String>, // Packages and Collections
    repo: &Repository,
) -> DotManResult<HashSet<String>> {
    let mut visited: Vec<String> = vec![];

    let mut result = HashSet::new();

    for pc in pcs {
        visited.push(pc.clone());

        let packages = packages(pc, repo)?;

        for p in packages {
            result.insert(p);
        }
    }

    let pkgs = result.clone();
    add_recursive(&pkgs, repo, &mut visited, &mut result)?;

    Ok(result)
}

fn add_recursive(
    pkgs: &HashSet<String>,
    repo: &Repository,
    visited: &mut Vec<String>,
    res: &mut HashSet<String>,
) -> DotManResult<()> {
    let visited_copy = &visited[..];

    let unique: Vec<String> = pkgs
        .iter()
        .filter(move |p| {
            for v in visited_copy {
                // This is cursed
                if **p == *v {
                    return false;
                }
            }
            true
        })
        .map(|p| p.clone())
        .collect();

    if unique.len() == 0 {
        return Ok(());
    }

    for p in unique {
        let pack = packages(&p, repo)?;
        for p in pack {
            visited.push(p.clone());
            res.insert(p.to_string());
        }
    }

    add_recursive(pkgs, repo, visited, res)
}

fn packages(package: &str, repo: &Repository) -> DotManResult<HashSet<String>> {
    for c in &repo.collections[..] {
        if c.name == package {
            let p = c.packages.clone();
            return Ok(str_vec_to_str_hash(&p));
        }
    }

    for p in &repo.packages[..] {
        if p.name == package {
            let mut deps = p.dependencies.clone();
            deps.push(p.name.clone());
            return Ok(str_vec_to_str_hash(&deps));
        }
    }

    Err(Error::UnknownPackage(package.to_owned()))
}

fn str_vec_to_str_hash(v: &Vec<String>) -> HashSet<String> {
    let mut set = HashSet::new();
    for i in v {
        set.insert(i.clone());
    }

    set
}
