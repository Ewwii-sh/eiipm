use super::{is_update_needed_for, load_db};
use colored::Colorize;
use log::info;
use std::error::Error;

pub fn check_package_updates(package_name: &Option<String>) -> Result<(), Box<dyn Error>> {
    let mut db = load_db()?;
    let mut pkg_needing_update: Vec<&String> = Vec::new();

    if let Some(name) = package_name {
        if db.packages.get_mut(name).is_some() {
            info!("> Checking for '{}' update", name.yellow().bold());
            let need_update = is_update_needed_for(&name)?;

            if need_update.0 {
                pkg_needing_update.push(name);
            }
        } else {
            info!("Package '{}' not found in database", name.yellow());
        }
    } else {
        info!("> Checking for updates in all packages...");
        for (name, ..) in db.packages.iter_mut() {
            info!("Checking '{}'", name.yellow().bold());
            let need_update = is_update_needed_for(&name)?;

            if need_update.0 {
                pkg_needing_update.push(name);
            }
        }
    }

    if !pkg_needing_update.is_empty() {
        info!("\nPackages needing updates:");
        for pkg in &pkg_needing_update {
            info!("  - {}", pkg);
        }
    } else {
        info!("{}", "\nAll packages are up to date!".green());
    }

    Ok(())
}
