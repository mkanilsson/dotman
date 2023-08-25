use std::{collections::HashSet, thread};

use colored::{ColoredString, Colorize};
use indicatif::ProgressBar;

use crate::{
    config::Config, errors::DotManResult, gitactions::GitWrapper, package, print, repo::Repository,
    required_packages,
};

pub fn install(conf: &Config, repo: &Repository, packages: &Vec<String>) -> DotManResult<()> {
    let deps_pb = ProgressBar::new_spinner();
    deps_pb.set_message(format!(
        "[{}] Loading dependencies...",
        "INFO".blue().bold()
    ));
    let packages = required_packages::gather_required_packages(packages, &repo)?;
    deps_pb.finish_with_message(format!(
        "[{}] Loading dependencies... DONE",
        "INFO".blue().bold()
    ));

    let packages_string = packages
        .iter()
        .map(|p| p.clone().italic().bold().to_string())
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
        .with_help_message("This will remove existing configurations")
        .prompt()?;

    if !result {
        print::info("Okay, exiting...");
        return Ok(());
    }

    // TODO: Make the required parent directories

    for pkg in packages {
        let pkg = repo.get_package_from_string(&pkg);
        let deps_pb = ProgressBar::new_spinner();
        deps_pb.set_message(format!(
            "[{}] Cloning {} from {} to {}...",
            "INFO".blue().bold(),
            pkg.name,
            pkg.url(),
            pkg.install_path()?
        ));

        // TODO: Check if the config exists and if it's a dotman repo
        //       If it is, use it, otherwise, remove it and clone
        GitWrapper::clone(&pkg.url(), &pkg.install_path()?)?;

        deps_pb.finish_with_message(format!(
            "[{}] Cloning {} from {} to {}... DONE",
            "INFO".blue().bold(),
            pkg.name,
            pkg.url(),
            pkg.install_path()?
        ));

        // TODO: Run `.dotman-postinstall` script
    }

    Ok(())
}
