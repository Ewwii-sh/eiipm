use clap::Parser;

/// Eiipm package manager for ewwii.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the package to install
    #[arg(short, long)]
    pub install: Option<String>,

    /// Name of the package to uninstall
    #[arg(short, long)]
    pub uninstall: Option<String>,

    /// Show debug logs
    #[arg(long)]
    pub debug: bool,

}