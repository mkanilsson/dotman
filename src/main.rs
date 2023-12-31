use clap::Parser;
use cli::{Commands, InstallUpdateArgs};
use config::Config;
use repo::Repository;

mod cli;
mod commands;
mod config;
mod errors;
mod gitactions;
mod package;
mod print;
mod remote;
mod repo;
mod required_packages;
mod script;
mod source;

fn main() {
    let cli = cli::Cli::parse();

    let conf = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            e.print_fatal();
            panic!();
        }
    };

    let repo = match Repository::load(&conf) {
        Ok(r) => r,
        Err(e) => {
            e.print_fatal();
            panic!();
        }
    };

    match &cli.command {
        Commands::Install {
            packages,
            yes,
            force,
            no_scripts,
        } => match commands::install_or_update(
            &conf,
            &repo,
            InstallUpdateArgs {
                yes,
                force,
                no_scripts,
            },
            packages,
        ) {
            Ok(_) => {}
            Err(e) => {
                e.print_error();
                return;
            }
        },
        Commands::Update {
            packages,
            yes,
            force,
            no_scripts,
        } => {
            match commands::install_or_update(
                &conf,
                &repo,
                InstallUpdateArgs {
                    yes,
                    force,
                    no_scripts,
                },
                packages,
            ) {
                Ok(_) => {}
                Err(e) => {
                    e.print_error();
                    return;
                }
            }
        }
        Commands::InstallEverything {
            yes,
            force,
            no_scripts,
        } => {
            let mut all_packages = vec![];
            for pkg in &repo.packages {
                all_packages.push(pkg.name.clone());
            }

            match commands::install_or_update(
                &conf,
                &repo,
                InstallUpdateArgs {
                    yes,
                    force,
                    no_scripts,
                },
                &all_packages,
            ) {
                Ok(_) => {}
                Err(e) => {
                    e.print_error();
                    return;
                }
            }
        }
        Commands::Search { query } => commands::search(&repo, &query),
        Commands::Inspect { package } => commands::inspect(&repo, &package),
    }
}
