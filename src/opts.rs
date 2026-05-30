use clap::{Parser, Subcommand};

/// Simple plugin manager for Ewwii.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Show debug logs
    #[arg(long, global = true)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize plugins.toml
    Init,

    /// Install everything in plugins.toml
    Install,

    /// Add a plugin to plugins.toml
    Add(AddArgs),

    /// Remove a plugin from plugins.toml and plugins/
    Remove {
        /// Plugin to remove
        plugin: String,
    },

    /// Update all plugins
    Update {
        /// Only update a singular plugin
        plugin: Option<String>
    },

    /// Clean entries in plugins/ that are not present in plugins.toml
    Clean,

    /// Clean the cache
    CacheClean,

    /// List all plugins
    List,
}

#[derive(Parser, Debug)]
pub struct AddArgs {
    /// Plugin to add. Format: "user/repo"
    pub plugin: String,
    /// Branch/tag/sha to reference
    #[arg(long = "ref")]
    pub ref_: Option<String>,
    /// Prefer prebuilt binary over building from source
    #[arg(long)]
    pub prebuilt: bool,
    /// Override build command
    #[arg(long)]
    pub build: Option<String>,
    /// Override artifact path
    #[arg(long)]
    pub artifact: Option<String>,
}
