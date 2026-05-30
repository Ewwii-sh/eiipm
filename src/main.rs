mod utils;
mod opts;
mod git;
mod schema;
mod functions;

use clap::Parser;
use opts::{Args, Commands};
use functions::{
    install::install_plugins,
    init::init_plugin_repo,
    add::add_plugin,
    remove::remove_plugin,
    clean::clean_cache,
    clean::clean_plugins,
    list::list_plugins,
    update::update_plugins,
};
use log::Level;

fn main() {
    let args = Args::parse();

    set_debug_levels(args.debug);
    if args.debug {
        log::info!("Debug logging enabled");
    }

    match args.command {
        Commands::Init => {
            if let Err(e) = init_plugin_repo() {
                log::error!("Failed to initialize plugin repository: {}", e);
            }
        }
        Commands::Install => {
            if let Err(e) = install_plugins() {
                log::error!("Failed to install plugins: {}", e);
            }
        }
        Commands::Add(add_args) => {
            if let Err(e) = add_plugin(add_args) {
                log::error!("Failed to add plugin: {}", e);
            }
        }
        Commands::Remove { plugin } => {
            if let Err(e) = remove_plugin(plugin) {
                log::error!("Failed to remove plugin: {}", e);
            }
        }
        Commands::Update { plugin: maybe_plugin } => {
            if let Err(e) = update_plugins(maybe_plugin) {
                log::error!("Failed to update plugins: {}", e);
            }
        }
        Commands::Clean => {
            if let Err(e) = clean_plugins() {
                log::error!("Failed to clean plguins: {}", e);
            }
        }
        Commands::CacheClean => {
            if let Err(e) = clean_cache() {
                log::error!("Failed to clean cache: {}", e);
            }
        }
        Commands::List => {
            if let Err(e) = list_plugins() {
                log::error!("Failed to list plugins: {}", e);
            }
        }
    }
}

fn set_debug_levels(debug_mode: bool) {
    let mut builder = env_logger::Builder::from_default_env();

    if debug_mode {
        builder
            .filter_level(log::LevelFilter::Debug)
            .format_timestamp_secs()
            .format_module_path(true)
            .format_level(true);
    } else {
        builder
            .format(|buf, record| {
                use std::io::Write;

                match record.level() {
                    Level::Warn => writeln!(buf, "[WARN] {}", record.args()),
                    Level::Error => writeln!(buf, "[ERROR] {}", record.args()),
                    _ => writeln!(buf, "{}", record.args()),
                }
            })
            .filter_level(log::LevelFilter::Info);
    }

    builder.init();
}
