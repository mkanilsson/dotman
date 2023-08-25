use clap::{command, Parser, Subcommand};

/// Manage dotfiles easially
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Skip confirmation
    #[arg(short = 'y', long)]
    yes: bool,

    /// Force install, this will override existing configurations
    #[arg(long)]
    force: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install selected packages, will update if already installed
    Install {
        #[clap(required = true)]
        packages: Vec<String>,
    },

    /// Search for packages and collections
    Search { query: String },

    /// View package information
    Inspect { package: String },

    /// Install every package
    InstallEverything,

    /// Updated selected packages, will install in not already installed
    Update {
        #[clap(required = true)]
        packages: Vec<String>,
    },
}
