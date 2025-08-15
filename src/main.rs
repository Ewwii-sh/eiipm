mod opts;
mod functions;

use opts::{Args, Commands};
use functions::install::install_package;
use functions::uninstall::uninstall_package;

use clap::Parser;
use log::Level;

fn main() {
    let args = Args::parse();

    if args.debug {
        log::info!("Debug logging enabled");
        set_debug_levels(true);
    }

    match args.command {
        Commands::Install { package } => {
            if let Err(e) = install_package(&package) {
                log::error!("Error installing '{}': {}", package, e);
            }
        }
        Commands::Uninstall { package } => {
            if let Err(e) = uninstall_package(&package) {
                log::error!("Error uninstalling '{}': {}", package, e);
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
