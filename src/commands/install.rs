use std::fs;

use colored::Colorize;
use indicatif::ProgressBar;

use crate::{
    config::{Config, GitUpdateType},
    errors::{DotManResult, Error, GitError},
    gitactions::GitWrapper,
    print,
    repo::Repository,
    required_packages, script,
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
        "Packages ({}) {} will be installed or updated.",
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

        let pp = print::Printer::new(pkg.name.clone());

        pp.info(&format!(
            "Looking if '{}' already exists...",
            install_path.italic()
        ));

        match GitWrapper::open(&url, &install_path) {
            Err(Error::Git(GitError::NotARepository(_))) => {
                print::info(&format!("Installing {}!", pkg.name.bold().italic()));
                pp.warning(&format!(
                    "'{}' exists but isn't a dotman repo, removing...",
                    install_path.italic()
                ));

                fs::remove_dir_all(install_path.clone())?;

                pp.info(&format!(
                    "Cloning {} from {} to {}... ",
                    pkg.name.italic().bold(),
                    url.italic(),
                    install_path.italic()
                ));

                GitWrapper::clone(&pkg.url(), &pkg.install_path()?)?;

                pp.success(&format!(
                    "{} cloned from {} to {}! ",
                    pkg.name.italic().bold(),
                    url.italic(),
                    install_path.italic()
                ));

                pp.info(&format!(
                    "Running `{}` script if it exists...",
                    ".dotman-postinstall".italic()
                ));
                script::run_postinstall(&install_path)?;

                print::success(&format!(
                    "{} has been successfully updated!",
                    pkg.name.bold().italic()
                ));
            }
            Err(e) => return Err(e),
            Ok(wrapper) => {
                print::info(&format!("Updating {}!", pkg.name.bold().italic()));
                pp.info(&format!(
                    "'{}' exists and is a dotman repo, updating instead...",
                    install_path.italic()
                ));

                let current_branch = wrapper.current_branch_name()?;
                if current_branch != "master" {
                    pp.info(&format!(
                        "Currently on '{}' branch. switching to '{}' branch...",
                        current_branch.italic(),
                        "master".bold()
                    ));
                    pp.warning("Changes won't take effect until you switch back to master!");
                    wrapper.checkout_branch("master")?;
                    pp.success(&format!("Switched to '{}' branch!", "master".bold()));
                }

                let remote = wrapper.get_remote_name()?;
                // FIXME: Handle merge confilcts somehow...
                match conf.git.update_type {
                    GitUpdateType::FetchRebase => {
                        pp.info("Fetching and rebasing changes...");
                        wrapper.fetch(&remote)?;
                        wrapper.rebase()?;
                        pp.success("Changes has been fetched and rebased!");
                    }
                    GitUpdateType::Pull => {
                        pp.info("Pulling changes...");
                        wrapper.pull(&remote)?;
                        pp.success("Changes has been pulled!");
                    }
                }

                pp.info(&format!(
                    "Running `{}` script if it exists...",
                    ".dotman-postupdate".italic()
                ));
                script::run_postupdate(&install_path)?;

                if current_branch != "master" {
                    pp.info(&format!(
                        "Switching back to '{}' branch...",
                        current_branch.italic(),
                    ));
                    wrapper.checkout_branch(&current_branch)?;
                    pp.success(&format!("Switched to '{}' branch!", current_branch.bold()));
                }

                print::success(&format!(
                    "{} has been successfully updated!",
                    pkg.name.bold().italic()
                ));
            }
        }
    }

    Ok(())
}
