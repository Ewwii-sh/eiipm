use clap::Parser;
use std::env;
use std::io::Write;
use log::Level;

mod opts;
mod functions;
use opts::Args;
use crate::functions::install::install_package;

fn main() {
    let args = Args::parse();

    set_debug_levels(args.debug);

    if let Some(install_pkg_name) = args.install.as_deref() {
        install_package(install_pkg_name);
    }

    if let Some(uninstall_pkg_name) = args.uninstall.as_deref() {
        log::debug!("Uninstalling package: {}", uninstall_pkg_name);
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
