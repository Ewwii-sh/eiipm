use clap::{Parser, Subcommand, Args};

/// Eiipm package manager for ewwii.
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(arg_required_else_help = true)]
pub struct PMArgs {
    /// Show debug logs
    #[arg(long, global = true)]
    pub debug: bool,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install a package
    #[command(alias = "i")]
    Install {
        /// Name of the package to install
        package: String,
    },
    /// Uninstall a package
    #[command(alias = "rm")]
    Uninstall {
        /// Name of the package to uninstall
        package: String,
    },
    /// Update a package or all packages
    #[command(alias = "up")]
    Update {
        /// Name of the package to update. Updates all if not provided.
        package: Option<String>,
    },
    /// List all installed packages
    #[command(alias = "l")]
    List(ListArgs),
    /// Clean a package or all package cache
    #[command(alias = "cc")]
    ClearCache {
        package: Option<String>,
    },
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Verbose output
    #[arg(long, short = 'v')]
    pub verbose: bool,

    /// Output just the total package count
    #[arg(long, short = 't')]
    pub total_count: bool,

    /// Query a package
    #[arg(short, long)]
    pub query: Option<String>,
}
