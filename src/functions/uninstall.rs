use colored::Colorize;
use dirs;
use log::{info};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

const DB_FILE: &str = ".config/eiipm/installed.toml";

#[derive(Deserialize, Serialize, Debug)]
struct PackageDB {
    packages: HashMap<String, InstalledPackage>,
}

#[derive(Deserialize, Serialize, Debug)]
struct InstalledPackage {
    repo_path: String,
    files: Vec<String>,
    pkg_type: String,
}

pub fn uninstall_package(package_name: &str) -> Result<(), Box<dyn Error>> {
    info!("> Uninstalling package '{}'", package_name.yellow().bold());

    let mut db = load_db()?;
    if let Some(pkg) = db.packages.remove(package_name) {
        for file in pkg.files {
            let path = PathBuf::from(file);
            if path.exists() {
                fs::remove_file(&path)?;
                info!("Removed file '{}'", path.display());
            }
        }
        save_db(&db)?;
        info!("Successfully uninstalled '{}'", package_name.yellow().bold());
    } else {
        info!("Package '{}' not found in database", package_name.yellow());
    }

    Ok(())
}

fn load_db() -> Result<PackageDB, Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let db_path = home_dir.join(DB_FILE);
    if db_path.exists() {
        let content = fs::read_to_string(&db_path)?;
        let db: PackageDB = toml::from_str(&content)?;
        Ok(db)
    } else {
        Ok(PackageDB { packages: HashMap::new() })
    }
}

fn save_db(db: &PackageDB) -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let db_path = home_dir.join(DB_FILE);
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(db)?;
    fs::write(db_path, content)?;
    Ok(())
}
