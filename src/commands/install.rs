use std::collections::HashSet;

use crate::{config::Config, errors::DotManResult, print, repo::Repository, required_packages};

pub fn install(conf: &Config, repo: &Repository, packages: &Vec<String>) -> DotManResult<()> {
    let packages = required_packages::gather_required_packages(packages, &repo)?;

    let packages_string = packages
        .iter()
        .map(|p| p.clone())
        .collect::<Vec<String>>()
        .join(", ");

    print::info(&format!(
        "Packages ({}) {} will be installed.",
        packages.len(),
        packages_string
    ));

    // FIXME: Handle OperationInterupted
    let result = inquire::Confirm::new("Do you want to procced?")
        .with_default(true)
        .prompt()?;

    if !result {
        print::info("Okay, exiting...");
        return Ok(());
    }

    Ok(())
}
