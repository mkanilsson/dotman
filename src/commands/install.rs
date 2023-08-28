use std::fs;

use colored::Colorize;
use indicatif::ProgressBar;

use crate::{
    cli::InstallUpdateArgs,
    config::{Config, GitUpdateType},
    errors::{DotManResult, Error, GitError},
    gitactions::GitWrapper,
    print,
    repo::Repository,
    required_packages, script,
};

pub fn install_or_update(
    conf: &Config,
    repo: &Repository,
    args: InstallUpdateArgs,
    packages: &Vec<String>,
) -> DotManResult<()> {
    let deps_pb = ProgressBar::new_spinner();
    deps_pb.set_message("Loading dependencies...");
    let packages = required_packages::gather_required_packages(packages, &repo)?;
    deps_pb.finish_with_message("Loading dependencies... DONE");

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

    if !(*args.yes) {
        // FIXME: Handle OperationInterupted
        let mut confirm = inquire::Confirm::new("Do you want to procced?").with_default(true);

        if *args.force {
            confirm = confirm.with_help_message("This might remove existing configurations");
        }

        let result = confirm.prompt()?;

        if !result {
            print::info("Okay, exiting...");
            return Ok(());
        }
    } else {
        print::warning("Running with no confirmation...");
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
                if !(*args.force) {
                    pp.fatal(&format!(
                        "'{}' exists but isn't a dotman repo, exiting...",
                        install_path.italic()
                    ));

                    unreachable!();
                }

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

                if !(*args.no_scripts) {
                    pp.info(&format!(
                        "Running `{}` script if it exists...",
                        ".dotman-postinstall".italic()
                    ));
                    script::run_postinstall(&install_path)?;
                } else {
                    pp.warning(&format!(
                        "Not running `{}` may require extra manual configuration...",
                        ".dotman-install".italic()
                    ));
                }

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

                if !(*args.no_scripts) {
                    pp.info(&format!(
                        "Running `{}` script if it exists...",
                        ".dotman-postupdate".italic()
                    ));
                    script::run_postupdate(&install_path)?;
                } else {
                    pp.warning(&format!(
                        "Not running `{}` may require extra manual configuration...",
                        ".dotman-postupdate".italic()
                    ));
                }

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
