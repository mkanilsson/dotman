use std::fs;

use colored::Colorize;
use indicatif::ProgressBar;

use crate::{
    config::{Config, GitUpdateType},
    errors::{DotManResult, Error, GitError},
    gitactions::GitWrapper,
    print,
    repo::Repository,
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
        let install_path = pkg.install_path()?;
        let url = pkg.url();

        print::info(&format!(
            "Looking if '{}' already exists...",
            install_path.italic()
        ));

        match GitWrapper::open(&url, &install_path) {
            Err(Error::Git(GitError::NotARepository(_))) => {
                print::warning(&format!(
                    "'{}' exists but isn't a dotman repo, removing...",
                    install_path.italic()
                ));

                fs::remove_dir_all(install_path.clone())?;

                print::info(&format!(
                    "Cloning {} from {} to {}... ",
                    pkg.name.italic().bold(),
                    url.italic(),
                    install_path.italic()
                ));

                GitWrapper::clone(&pkg.url(), &pkg.install_path()?)?;

                print::success(&format!(
                    "{} cloned from {} to {}! ",
                    pkg.name.italic().bold(),
                    url.italic(),
                    install_path.italic()
                ));

                // TODO: Run `.dotman-postinstall` script
            }
            Err(e) => return Err(e),
            Ok(wrapper) => {
                print::info(&format!(
                    "'{}' exists and is a dotman repo, updating instead...",
                    install_path.italic()
                ));

                let current_branch = wrapper.current_branch_name()?;
                if current_branch != "master" {
                    print::info(&format!(
                        "Currently on '{}' branch. switching to '{}' branch...",
                        current_branch.italic(),
                        "master".bold()
                    ));
                    wrapper.checkout_branch("master")?;
                    print::success(&format!("Switched to '{}' branch!", "master".bold()));
                }

                let remote = wrapper.get_remote_name()?;
                // FIXME: Handle merge confilcts somehow...
                match conf.git.update_type {
                    GitUpdateType::FetchRebase => {
                        print::info("Fetching and rebasing changes...");
                        wrapper.fetch(&remote)?;
                        wrapper.rebase()?;
                        print::success("Changes has been fetched and rebased!");
                    }
                    GitUpdateType::Pull => {
                        print::info("Pulling changes...");
                        wrapper.pull(&remote)?;
                        print::success("Changes has been pulled!");
                    }
                }

                // TODO: Run `.dotman-postupdate` script

                if current_branch != "master" {
                    print::info(&format!(
                        "Switching back to '{}' branch...",
                        current_branch.italic(),
                    ));
                    wrapper.checkout_branch(&current_branch)?;
                    print::success(&format!("Switched to '{}' branch!", current_branch.bold()));
                }
            }
        }
    }

    Ok(())
}
