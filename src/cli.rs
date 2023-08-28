use clap::{command, Parser, Subcommand};

/// Manage dotfiles easially
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install selected packages, will update if already installed
    Install {
        #[clap(required = true)]
        packages: Vec<String>,

        /// Skip confirmation
        #[arg(short = 'y', long)]
        yes: bool,

        /// Force install, this will override existing configurations
        #[arg(long)]
        force: bool,

        /// Don't run .dotman-postinstall or .dotman-postupdate script
        #[arg(long)]
        no_scripts: bool,
    },

    /// Search for packages and collections
    Search { query: String },

    /// View package information
    Inspect { package: String },

    /// Install every package avaliable in the repository
    InstallEverything {
        /// Skip confirmation
        #[arg(short = 'y', long)]
        yes: bool,

        /// Force install, this will override existing configurations
        #[arg(long)]
        force: bool,

        /// Don't run .dotman-postinstall or .dotman-postupdate script
        #[arg(long)]
        no_scripts: bool,
    },

    /// Updated selected packages, will install in not already installed
    Update {
        #[clap(required = true)]
        packages: Vec<String>,

        /// Skip confirmation
        #[arg(short = 'y', long)]
        yes: bool,

        /// Force install, this will override existing configurations
        #[arg(long)]
        force: bool,

        /// Don't run .dotman-postinstall or .dotman-postupdate script
        #[arg(long)]
        no_scripts: bool,
    },
}

pub struct InstallUpdateArgs<'a> {
    pub yes: &'a bool,
    pub force: &'a bool,
    pub no_scripts: &'a bool,
}
