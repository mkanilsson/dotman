use colored::Colorize;

use crate::{print, repo::Repository};

pub fn search(repo: &Repository, query: &str) {
    print::info(&format!(
        "Searching through {} packages and collections for '{}'",
        (repo.collections.len() + repo.packages.len())
            .to_string()
            .bold(),
        query.bold()
    ));

    for pkg in &repo.packages {
        let matches = [
            &pkg.name,
            &pkg.description,
            &pkg.install_path,
            &pkg.repo.url(),
        ]
        .iter()
        .map(|i| matches(query, i))
        .fold(false, |res, other| res || other);

        if matches {
            pkg.pprint();
        }
    }

    for col in &repo.collections {
        let matches = [&col.name, &col.description]
            .iter()
            .map(|i| matches(query, i))
            .fold(false, |res, other| res || other);

        if matches {
            col.pprint();
        }
    }
}

fn matches(needle: &str, haystack: &str) -> bool {
    let haystack = clean(haystack);
    let needle = clean(needle);

    haystack.contains(&needle)
}

fn clean(s: &str) -> String {
    s.to_lowercase()
        .trim()
        .replace("\n", "")
        .replace("\t", "")
        .replace(",", "")
}
