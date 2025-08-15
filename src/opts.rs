use clap::{Parser, Subcommand};

/// Eiipm package manager for ewwii.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Show debug logs
    #[arg(long)]
    pub debug: bool,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install a package
    Install {
        /// Name of the package to install
        package: String,
    },
    /// Uninstall a package
    Uninstall {
        /// Name of the package to uninstall
        package: String,
    },
}