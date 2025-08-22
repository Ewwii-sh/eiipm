mod functions;
mod git;
mod opts;
mod other;

use functions::{
    checkupdate::check_package_updates, clearcache::clean_package_cache, install::install_package,
    list::list_packages, listcache::list_all_cache, purgecache::purge_cache,
    search::search_package, uninstall::uninstall_package, update::update_package,
};
use opts::{Commands, PMArgs};
use other::{confirm_action::confirm, run_checks::check_eiipm_in_path};

use clap::Parser;
use log::{Level, error, info};

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
        Commands::Update { package } => match &package {
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
        },
        Commands::List(list_args) => {
            if let Err(e) = list_packages(list_args) {
                error!("Error listing packages: {}", e);
            }
        }
        Commands::ClearCache { package, flags } => {
            let question = match package {
                Some(ref pkg) => format!("Delete cache of '{}' permanently?", pkg),
                None => "Delete ALL caches permanently?".to_string(),
            };

            let user_confirmed: bool;

            if !flags.force {
                user_confirmed = confirm(&question);
            } else {
                user_confirmed = true;
            }

            if user_confirmed {
                if let Err(e) = clean_package_cache(package) {
                    error!("Error clearing package cache: {}", e);
                }
            } else {
                match package {
                    Some(pkg) => info!("Cache clear canceled for package '{}'", pkg),
                    None => info!("Cache clear canceled for all packages"),
                }
            }
        }
        Commands::CheckUpdates { package } => match &package {
            Some(name) => {
                if let Err(e) = check_package_updates(&Some(name.clone())) {
                    error!("Error checking for updates in '{}'. Caused by: {}", name, e);
                }
            }
            None => {
                if let Err(e) = check_package_updates(&None) {
                    error!("Error checking for updates in all packages: {}", e);
                }
            }
        },
        Commands::ListCacheDir => {
            if let Err(e) = list_all_cache() {
                error!("Error listing cache: {}", e);
            }
        }
        Commands::PurgeCache => {
            if let Err(e) = purge_cache() {
                error!("Error purging cache: {}", e);
            }
        }
        Commands::Search { package, flags } => {
            if let Err(e) = search_package(&package, flags) {
                error!("Error searching for '{}'. Error: {}", package, e);
            }
        }
    }

    check_eiipm_in_path();
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
