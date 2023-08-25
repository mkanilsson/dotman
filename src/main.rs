use clap::Parser;
use cli::Commands;
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
mod source;

#[async_std::main]
async fn main() {
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

    println!("{:#?}", repo);

    let packages_to_install = vec!["hyprland".to_owned()];
    match required_packages::gather_required_packages(&packages_to_install, &repo) {
        Ok(pkgs) => println!("Packages: {:#?}", pkgs),
        Err(e) => {
            e.print_fatal();
            panic!();
        }
    }

    match &cli.command {
        Commands::Install { packages: _ } => print::fatal("Install hasn't been implemented yet"),
        Commands::InstallEverything => {
            print::fatal("Install everything hasn't been implemented yet")
        }
        Commands::Search { query: _ } => print::fatal("Search hasn't been implemented yet"),
        Commands::Update { packages: _ } => print::fatal("Update hasn't been implemented yet"),
    }
}
