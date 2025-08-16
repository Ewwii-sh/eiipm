mod opts;
mod functions;

use opts::{PMArgs, Commands};
use functions::{
    install::install_package,
    uninstall::uninstall_package,
    update::update_package,
    list::list_packages,
    clearcache::clean_package_cache,
};

use clap::Parser;
use log::{info, error, Level};

fn main() {
    let args = PMArgs::parse();

    set_debug_levels(args.debug);

    if args.debug {
        info!("Debug logging enabled");
    }

    match args.command {
        Commands::Install { package } => {
            if let Err(e) = install_package(&package) {
                error!("Error installing '{}': {}", package, e);
            }
        }
        Commands::Uninstall { package } => {
            if let Err(e) = uninstall_package(&package) {
                error!("Error uninstalling '{}': {}", package, e);
            }
        }
        Commands::Update { package } => {
            match &package {
                Some(name) => {
                    if let Err(e) = update_package(&Some(name.clone())) {
                        error!("Error updating '{}'. Caused by: {}", name, e);
                    }
                }
                None => {
                    if let Err(e) = update_package(&None) {
                        error!("Error updating all packages: {}", e);
                    }
                }
            }
        }
        Commands::List(list_args) => {
            if let Err(e) = list_packages(list_args) {
                error!("Error listing packages: {}", e);
            }
        }
        Commands::ClearCache { package } => {
            if let Err(e) = clean_package_cache(package) {
                error!("Error clearing package cache: {}", e);
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
        builder.format(|buf, record| {
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
