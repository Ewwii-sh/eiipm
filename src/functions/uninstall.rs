use super::{load_db, save_db};
use colored::Colorize;
use log::info;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn uninstall_package(package_name: &str) -> Result<(), Box<dyn Error>> {
    info!("> Uninstalling package '{}'", package_name.yellow().bold());

    let mut db = load_db()?;
    if let Some(pkg) = db.packages.remove(package_name) {
        for file in pkg.installed_files {
            let path = PathBuf::from(file);
            if path.exists() {
                fs::remove_file(&path)?;
                info!("Removed file '{}'", path.display());
            }
        }
        save_db(&db)?;
        info!(
            "Successfully uninstalled '{}'",
            package_name.yellow().bold()
        );
    } else {
        info!("Package '{}' not found in database", package_name.yellow());
    }

    Ok(())
}
