use super::{InstalledPackage, save_db, load_db}; 
use log::{info, debug};
use std::error::Error;
use std::path::PathBuf;
use std::fs;
use colored::Colorize;

pub fn clean_package_cache(package_name: Option<String>) -> Result<(), Box<dyn Error>> {
    let mut db = load_db()?;

    if let Some(name) = package_name {
        if let Some(pkg) = db.packages.get_mut(&name) {
            info!("> Clearing '{}' cache", name.yellow().bold());
            clear_file_cache(pkg, &name)?;
        } else {
            info!("Package '{}' not found in database", name.yellow());
        }
    } else {
        info!("> Clearing all package cache...");
        for (name, pkg) in db.packages.iter_mut() {
            info!("Clearing '{}' cache", name.yellow().bold());
            clear_file_cache(pkg, name)?;
        }
    }

    save_db(&db)?;
    Ok(())
}

fn clear_file_cache(pkg: &mut InstalledPackage, package_name: &str) -> Result<(), Box<dyn Error>> {
    let repo_path = PathBuf::from(&pkg.repo_path);
    
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let cache_root = home_dir.join(".eiipm/cache");

    if !repo_path.exists() {
        info!("Cache of package '{}' doesn't exist. Skipping...", package_name);
        return Ok(());
    } else {
        debug!("Running catastrophe preventer code before removing cache.");
        if !repo_path.starts_with(cache_root.as_path()) {
            return Err(format!("Refusing to delete outside cache: {}", repo_path.display()).into());
        }

        match fs::remove_dir_all(repo_path) {
            Ok(_) => info!("Successfully cleared '{}' cache", package_name.yellow().bold()),
            Err(e) => {
                return Err(format!(
                    "Error removing cache of '{}'. Caused by: {}", 
                    package_name,
                    e
                ).into());
            },
        }
    }

    Ok(())
}